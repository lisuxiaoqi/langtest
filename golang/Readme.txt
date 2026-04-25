## 说明
本项目用于Go语言的功能测试

## 测试指令


```
//查看所有package
go list ./...

//查看指定目录package
go list .

//测试所有package，不缓存
go test -v -count=1 ./...

//测试指定package
go test github.com/lisuxiaoqi/gotest

//查看所有测试函数
go test -list .

//测试指定文件
go test -v print_test.go

//测试mod下指定函数
go test -v -run TestStructNil

```
