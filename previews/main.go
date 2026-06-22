// Package main is the main package
package main

import (
	_ "embed"
	"fmt"
)

// source is the source file.
//go:embed main.go
var source string

func main() {
	_ = true
	const a int = 1
	b := 2
	fmt.Printf("%d + %d = %d\n", a, b, a + b)

	numbers := []int{1}
	numbers = append(numbers, 2, 3)

	for _, number := range numbers {
		fmt.Println(number)
	}
}
