use x86_64::instructions::segmentation;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;
use bit_field::BitField;

pub type HandlerFunc = extern "C" fn() -> !;

pub struct Idt([Entry; 16]);

impl Idt {
    pub fn new() -> Self {
        Idt([Entry::missing(); 16])
    }

    pub fn set_handler(&mut self, entry: usize, handler: HandlerFunc) -> &mut EntryOptions {
        self.0[entry] = Entry::new(segmentation::cs(), handler);
        &mut self.0[entry].options
    }

    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lidt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: x86_64::VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr)};
    }

}


#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_hight: u32,
    reserved: u32,
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector: gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_hight: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }
    }

    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_hight: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut option: u16 = 0;
        option.set_bits(9..12, 0b111);
        EntryOptions(option)
    }

    fn new() -> Self {
        let mut options = Self::new();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable:bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self{
        self.0.set_bits(13..15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self{
        self.0.set_bits(0..3,index);
        self
    } 
}