## 目录结构
采用rust lib中集成测试的方式。（仅lib支持集成测试）
项目文件为lib
测试代码在tests文件夹中

## 测试方式
```
cargo test --test hello_test -- --show-output
```
* --test指运行集成测试
* hello_test是测试文件名称
* --之前的参数是传递给cargo test本身的。 --之后的参数是传递给测试二进制文件的
> 测试二进制文件是由cargo test执行时生成的一个二进制文件，会调用具体的测试函数。测试二进制文件由具体的测试框架生成
> rust中主要的测试框架是libtest。通过cargo test -- --help将显示libtest框架支持的一些通用参数
* --show--output是传递给测试二进制文件的参数，表示显示测试代码的输出，如代码中调用了println的输出