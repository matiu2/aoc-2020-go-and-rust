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
	a, b, err := day1_part1(dat)
	check(err)
	log.Printf("Day 1 Part 1: %d + %d = 2020 && %d * %d = %d", a, b, a, b, a*b)

	a, b, c, err := day1_part2(dat)
	check(err)
	log.Printf("Day 1 Part 2: %d + %d + %d = 2020 && %d * %d * %d = %d", a, b, c, a, b, c, a*b*c)
}
