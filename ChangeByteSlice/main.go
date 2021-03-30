package main

import "fmt"

func main() {
	for i := 0; i < 100_000_000; i++ {
		data := []byte{1, 2, 3, 4, 5, 6, 7, 8, 9}
		length := int16(len(data))
		data = append([]byte{0, 0}, data...)
		data[0], data[1] = uint8(length&0xff), uint8(length>>8)
	}
	fmt.Println("ok")
}
