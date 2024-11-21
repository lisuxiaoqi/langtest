package main

import (
	"fmt"
	"math/rand"
	"sync"
	"testing"
	"time"
)

/*
用生产者和消费者模型，采用sync.cond
*/
var queue []int
var cond = sync.NewCond(&sync.Mutex{})

func Producer() {
	for i := 0; i < 5; i++ {
		cond.L.Lock()
		v := rand.Intn(100)
		fmt.Printf("Produced: %d\n", v)
		queue = append(queue, v)
		cond.Signal()
		cond.L.Unlock()
		time.Sleep(2 * time.Second)
	}
}

func Consumer() {
	for {
		cond.L.Lock()
		for len(queue) == 0 {
			cond.Wait()
		}
		i := queue[0]
		queue = queue[1:]
		fmt.Printf("Consumed: %d\n", i)
		cond.L.Unlock()
		time.Sleep(1 * time.Second)
	}
}

func TestCond(t *testing.T) {
	go Producer()
	go Consumer()
	time.Sleep(10 * time.Second)
}
