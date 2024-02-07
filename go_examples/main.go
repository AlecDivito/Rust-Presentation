package main

import (
	"fmt"
	"sync"
)

func main() {
	var counter = 0
	var wg sync.WaitGroup
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			counter++
		}()
	}
	wg.Wait()
	fmt.Println("Counter value:", counter)
}
