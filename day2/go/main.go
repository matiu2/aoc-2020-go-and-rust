package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
)

func main() {
	data, err := ioutil.ReadFile("../input.txt")
	if err != nil {
		panic(err)
	}

	valid_count := 0
	for _, line := range bytes.Split(data, []byte("\n")) {
		if len(line) == 0 {
			// Ignore empty lines
			continue
		}
		rule, err := parse_rule(string(line))
		if err != nil {
			log.Fatalf("Couldn't parse rule: %s: %v", line, err)
		}
		if rule.is_valid() {
			valid_count++
		}
	}

	fmt.Printf("Day 2 Part 1 - valid rule count = %d\n", valid_count)
}
