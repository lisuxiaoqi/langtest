package main

import (
	"fmt"
	"math/rand"
	"sync"
	"testing"
	"time"
)

/*
channel和wait.group的基本用法
以及for, select的用法
*/
func TestChannel(t *testing.T) {
	wg := sync.WaitGroup{}
	q := make(chan struct{})
	wg.Add(1)
	go func() {
		timer := time.NewTimer(0 * time.Second)
		for {
			select {
			case t := <-timer.C:
				fmt.Println("Get Timer:", t)
			case <-q:
				fmt.Println("Quite routine")
				wg.Done()
			}
		}
	}()

	time.Sleep(3 * time.Second)
	q <- struct{}{}
	wg.Wait()
}

func TestRandChannel(t *testing.T) {
	const producerCount = 5
	numbers := make(chan int, 100) // 用于传递随机数的 channel
	var wg sync.WaitGroup          // 用于等待生产者完成

	// 启动 5 个生产者协程
	for i := 0; i < producerCount; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done() // 确保协程结束时调用 Done

			count := rand.Intn(5) + 1 // 随机生成 1 到 10 个随机数
			fmt.Printf("Producer %d generating %d numbers\n", id, count)
			for j := 0; j < count; j++ {
				num := rand.Intn(5) // 随机数范围 0 到 99
				numbers <- num
				fmt.Printf("Producer %d sent: %d\n", id, num)
			}
		}(i)
	}

	wg.Wait() // 等待所有生产者协程完成
	close(numbers)

	// 消费者从 channel 接收数据并计算总和
	sum := 0
	for num := range numbers {
		sum += num
		fmt.Printf("Consumer received: %d, current sum: %d\n", num, sum)
	}

	fmt.Printf("Final sum: %d\n", sum)
}

// close之后的channel依然可以读取，for i := range ch  就会在channel close之后退出
func Test_Close_Channel(t *testing.T) {
	ch := make(chan int, 10)
	wg := sync.WaitGroup{}
	wg.Add(1)
	go func() {
		defer wg.Done()
		for i := 0; i < 10; i++ {
			select {
			case ch <- i:
			default:
				t.Log("Value dropped", i)
			}
		}
	}()

	wg.Wait()
	close(ch)
	for i := range ch {
		t.Log("Received", i)
	}
}

// 多个routine同时监听一个channel，只会有一个routine接收到数据
// 这是一种**竞争接收（competing receivers）**的行为。
// 但是close channel除外，close时，所有routine都会收到信号
func Test_Multi_Listen(t *testing.T) {
	c := make(chan int)

	wg := sync.WaitGroup{}
	num := 1
	//num = 5
	for i := 0; i < num; i++ {
		wg.Add(1)
		go func(id int) {
			defer func() {
				wg.Done()
			}()

			for {
				select {
				case <-c:
					t.Log(id, "received")
					return
				default:
				}
			}
		}(i)
	}
	// data会被竞争接收
	c <- 0
	// close会被所有的routine接收
	//close(c)
	wg.Wait()
	t.Log("Quit main route")
}
