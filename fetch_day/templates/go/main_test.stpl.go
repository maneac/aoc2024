package main

import (
	"reflect"
	"testing"
)

var contents = readData("../../data")

func TestParseContents(t *testing.T) {
	tests := map[string]struct {
		input    string
		expected *Input
	}{
		"example": {
			input:    "",
			expected: exampleData(),
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := parseContents(test.input)

			if !reflect.DeepEqual(test.expected, actual) {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     *Input
		expected int
	}{
		"example": {
			data:     exampleData(),
			expected: 0,
		},
		"actual": {
			data:     parseContents(contents),
			expected: part1Solution,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := test.data.part1()

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := map[string]struct {
		data     *Input
		expected int
	}{
		"example": {
			data:     exampleData(),
			expected: 0,
		},
		"actual": {
			data:     parseContents(contents),
			expected: part2Solution,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := test.data.part2()

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func BenchmarkParseContents(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if parseContents(contents) == nil {
			b.FailNow()
		}
	}
}

func BenchmarkPart1(b *testing.B) {
	data := parseContents(contents)
	for i := 0; i < b.N; i++ {
		if data.part1() != part1Solution {
			b.FailNow()
		}
	}
}

func BenchmarkPart2(b *testing.B) {
	data := parseContents(contents)
	for i := 0; i < b.N; i++ {
		if data.part2() != part2Solution {
			b.FailNow()
		}
	}
}

func BenchmarkTotal(b *testing.B) {
	for i := 0; i < b.N; i++ {
		data := parseContents(contents)
		if data.part1() != part1Solution {
			b.FailNow()
		}
		if data.part2() != part2Solution {
			b.FailNow()
		}
	}
}

func exampleData() *Input {
	return &Input{}
}
