# OStrain
根据Blog_OS博文使用rust编写一个简易的OS。

## 第二篇文章 “最小化内核” 遇到的坑
```
cargo install bootimage
```
命令会安装新版本的bootimage工具，导致编译失败，更改为以下命令：
```
cargo install bootimage --version "^0.7.3"
```