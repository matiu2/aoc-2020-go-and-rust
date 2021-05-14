package main

import "testing"

func TestDay1Part1(t *testing.T) {
	input := `1721
979
366
299
675
1456`
	a1, b1, err := day1_part1([]byte(input))
	if err != nil {
		panic(err)
	}
	a2, b2 := 299, 1721
	if a1+b1 != 2020 {
		t.Errorf("Expected %d + %d = 2020 but got %d + %d = %d", a2, b2, a1, b1, a1+b1)
	}
}

func TestDay1Part2(t *testing.T) {
	input := `1721
979
366
299
675
1456`
	a, b, c, err := day1_part2([]byte(input))
	if err != nil {
		panic(err)
	}
	if a+b+c != 2020 {
		t.Errorf("Expected numbers that sum to 2020 but got %d + %d + %d = %d", a, b, c, a+b+c)
	}
}
