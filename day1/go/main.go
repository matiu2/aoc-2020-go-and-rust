package main

import (
	"io/ioutil"
	"log"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	dat, err := ioutil.ReadFile("../input.txt")
	check(err)
	a, b, err := day1(dat)
	check(err)
	log.Printf("Day 1 Answer: %d + %d = 2020 && %d * %d = %d", a, b, a, b, a*b)
}
