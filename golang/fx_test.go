package main

import (
	"fmt"
	"go.uber.org/fx"
	"testing"
)

/*
Provide仅仅是注册类型
类型注册后，如果没有地方引用，该类型不会被实例化，生命周期也不会被触发
Invoke的目的就是显式的初始化必要的类型，进一步触发其生命周期。
简单理解Provide准备了一辆车，需要Invoke来启动
*/
type Client struct {
}

func ClientConstructor() Client {
	return Client{}
}

func (c *Client) Start() {
	fmt.Println("Start Client")
}
func (c *Client) Stop() {
	fmt.Println("Stop Client")
}

func TestInvoke(t *testing.T) {
	opt1 := fx.Provide(fx.Annotate(
		ClientConstructor,
		fx.OnStart(func(client Client) {
			client.Start()
		}),
		fx.OnStop(func(client Client) {
			client.Stop()
		}),
	))

	opt2 := fx.Invoke(func(Client) {
	})
	fx.New(opt1, opt2).Run()

}

// lifecycle hook与Client的生命周期相关
// 在Onstart, Onstop中的参数无关
func TestInvoke2(t *testing.T) {
	opt1 := fx.Provide(fx.Annotate(
		ClientConstructor,
		fx.OnStart(func() {
			fmt.Println("hello")
		}),
		fx.OnStop(func() {
			fmt.Println("hello")
		}),
	))
	opt2 := fx.Invoke(func(Client) {
	})
	fx.New(opt1, opt2).Run()
}
