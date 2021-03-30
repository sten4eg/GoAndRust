package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

var Val int64

func main() {

	var wg sync.WaitGroup
	for i := 0; i < 20; i++ {
		go addCounter(&wg)
	}
	wg.Wait()
	fmt.Println(Val)
}

func addCounter(wg *sync.WaitGroup) {
	wg.Add(1)
	for i := 0; i < 1_000_000; i++ {
		atomic.AddInt64(&Val, 1)
	}
	wg.Done()
}
