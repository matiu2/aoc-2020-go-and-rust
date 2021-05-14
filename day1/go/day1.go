package main

import (
	"bytes"
	"errors"
	"strconv"
)

func day1_part1(input []byte) (int, int, error) {
	num_map := make(map[int]int)
	for _, line := range bytes.Split(input, []byte("\n")) {
		// Skip blank lines
		if len(line) == 0 {
			continue
		}
		// Convert the line to an int
		number, err := strconv.Atoi(string(line))
		// If we were unable to convert a line, return the error
		if err != nil {
			return 0, 0, err
		}
		// Store the number in our map
		num_map[number]++
	}
	for y := range num_map {
		x := 2020 - y
		if _, ok := num_map[x]; ok {
			return x, y, nil
		}
	}
	return 0, 0, errors.New("couldn't find a pair that matched")
}

func day1_part2(input []byte) (int, int, int, error) {
	nums := make([]int, 0)
	for _, line := range bytes.Split(input, []byte("\n")) {
		if len(line) == 0 {
			continue
		}
		num, err := strconv.Atoi(string(line))
		if err != nil {
			return 0, 0, 0, err
		}
		nums = append(nums, num)
	}
	for _, a := range nums {
		for _, b := range nums {
			for _, c := range nums {
				if a+b+c == 2020 {
					return a, b, c, nil
				}
			}
		}
	}
	return 0, 0, 0, errors.New("unable to find a triple that sums to 2020")
}
