package main

import "testing"

func TestRuleIsValid(t *testing.T) {
	// This rule should be valid because it doesn't have enough 'a's
	rule := Rule{
		min:      4,
		max:      6,
		chr:      "a",
		password: "abababb",
	}
	if rule.is_valid() {
		t.Errorf("Rule (%v) should be invalid", rule)
	}

	// This rule should be invalid, because it has too many 'Z's
	rule = Rule{
		min:      1,
		max:      3,
		chr:      "Z",
		password: "aZaZaZaZaZaZaZ",
	}

	if rule.is_valid() {
		t.Errorf("Rule (%v) should be invalid", rule)
	}

	// This rule should be valid though
	rule = Rule{
		min:      1,
		max:      30,
		chr:      "Z",
		password: "aZaZaZaZaZaZaZ",
	}

	if !rule.is_valid() {
		t.Errorf("Rule (%v) should be valid", rule)
	}
}
