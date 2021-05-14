package main

import (
	"bytes"
	"errors"
	"strconv"
)

func day1(input []byte) (int, int, error) {
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
	return 0, 0, errors.New("Couldn't find a pair that matched")
}
