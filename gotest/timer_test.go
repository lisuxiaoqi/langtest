package main

import (
	"fmt"
	"sync"
	"testing"
	"time"
)

/*
NewTimer在间隔d秒之后，触发一次timer事件，仅触发一次

测试：d=0时会发生什么

结论：timer会被触发一次，但不一定是马上触发，有一定微小延迟
*/
func TestTimerZero(t *testing.T) {
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
