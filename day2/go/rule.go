package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Rule struct {
	min      int
	max      int
	chr      string
	password string
}

func (r *Rule) is_valid() bool {
	count := strings.Count(r.password, r.chr)
	return count >= r.min && count <= r.max
}

func parse_rule(line string) (Rule, error) {
	// A rule's format is:
	// "1-3 c: password"

	// First we split into
	// ["1, "3", "c", "password"]

	minus := strings.Index(line, "-")
	if minus < 0 {
		return Rule{}, fmt.Errorf("unable to find the minus sign for rule: %s", line)
	}

	space := minus + strings.Index(line[minus:], " ")
	if space < 0 {
		return Rule{}, fmt.Errorf("unabble to find the 'space' for rule: %s", line)
	}

	colon := space + strings.Index(line[space:], ": ")
	if colon < 0 {
		return Rule{}, fmt.Errorf("unable to find the ':' for rule: %s", line)
	}

	mins := line[:minus]
	maxs := line[(minus + 1):space]
	chr := line[(space + 1):colon]
	password := line[(colon + 2):]

	// Parse the ints
	min, err := strconv.Atoi(mins)
	if err != nil {
		return Rule{}, fmt.Errorf("unable to parse integer for the minimum from rule: %s", line)
	}
	max, err := strconv.Atoi(maxs)
	if err != nil {
		return Rule{}, fmt.Errorf("unable to parse integer for the maximum from rule: %s", line)
	}

	return Rule{
		min:      min,
		max:      max,
		chr:      chr,
		password: password,
	}, nil
}
