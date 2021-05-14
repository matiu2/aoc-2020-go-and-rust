package main

import "testing"

func TestDay1(t *testing.T) {
	input := `1721
979
366
299
675
1456`
	a1, b1, err := day1([]byte(input))
	if err != nil {
		panic(err)
	}
	a2, b2 := 299, 1721
	if a1+b1 != 2020 {
		t.Errorf("Expected %d + %d = 2020 but got %d + %d = %d", a2, b2, a1, b1, a1+b1)
	}
}
