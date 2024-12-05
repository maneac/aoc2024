package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

const (
	part1Solution = 0
	part2Solution = 0
)

type Input struct{}

func main() {
	contents := readData("./data")
	data := parseContents(contents)
	fmt.Printf("Part 1: %v\n", data.part1())
	fmt.Printf("Part 2: %v\n", data.part2())
}

func readData(dir string) string {
	contents, err := os.ReadFile(filepath.Join(dir, "<%= &self.package_name %>.txt"))
	if err != nil {
		panic(err)
	}

	return strings.TrimSpace(string(contents))
}

func parseContents(contents string) *Input {
	panic("todo")
	return &Input{}
}

func (i Input) part1() int {
	panic("todo")
	return 0
}

func (i Input) part2() int {
	panic("todo")
	return 0
}
