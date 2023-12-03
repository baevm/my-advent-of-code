package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"slices"
	"strconv"
	"unicode"
)

// https://adventofcode.com/2023/day/3
func main() {
	lines, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(lines)

	fmt.Println("1. Result: ", res1)

	res2 := solve_2(lines)

	fmt.Println("2. Result: ", res2)
}

// Check is current char is dot or number
func isDotOrNumber(ch byte) bool {
	return ch == '.' || unicode.IsNumber(rune(ch))
}

func isStar(ch byte) bool {
	return ch == '*'
}

func solve_1(lines []string) int {
	res := 0

	for i, line := range lines {

		num := ""
		isAdjacent := false

		for j, ch := range line {
			if unicode.IsNumber(ch) {
				num += string(ch)
			}

			// Check if we reached end of the number
			// j == len(lines[i]) -1 <--- used when number is last symbol on the line
			if !unicode.IsNumber(ch) || j == len(lines[i])-1 {
				if isAdjacent {
					currNum, err := strconv.Atoi(num)

					if err != nil {
						panic(err)
					}

					res += currNum
				}

				num = ""
				isAdjacent = false
				continue
			}

			// Start checking clockwise
			// check top
			if i > 0 && !isDotOrNumber(lines[i-1][j]) {
				isAdjacent = true
			}

			// check top right
			if i > 0 && j+1 < len(lines[i]) && !isDotOrNumber(lines[i-1][j+1]) {
				isAdjacent = true
			}

			// check right
			if j+1 < len(lines[i]) && !isDotOrNumber(lines[i][j+1]) {
				isAdjacent = true
			}

			// check bottom right
			if i+1 < len(lines) && j+1 < len(lines[i]) && !isDotOrNumber(lines[i+1][j+1]) {
				isAdjacent = true
			}

			// check bottom
			if i+1 < len(lines) && !isDotOrNumber(lines[i+1][j]) {
				isAdjacent = true
			}

			// check bottom left
			if i+1 < len(lines) && j > 0 && !isDotOrNumber(lines[i+1][j-1]) {
				isAdjacent = true
			}

			// check left
			if j > 0 && !isDotOrNumber(lines[i][j-1]) {
				isAdjacent = true
			}

			//check top left
			if i > 0 && j > 0 && !isDotOrNumber(lines[i-1][j-1]) {
				isAdjacent = true
			}
		}
	}

	return res
}

func solve_2(lines []string) int {
	res := 0

	for y, line := range lines {

		for x := range line {
			if isStar(lines[y][x]) {
				n := getNeighbouringNumbers(lines, x, y)

				if len(n) == 2 {
					res += n[0] * n[1]
				}
			}
		}
	}

	return res
}

func getNeighbouringNumbers(input []string, x0 int, y0 int) []int {
	var n []int

	for x := -1; x <= 1; x++ {
		for y := -1; y <= 1; y++ {
			i, ok := getNumberWithinCoordinates(input, x0+x, y0+y)

			if ok && !slices.Contains(n, i) {
				n = append(n, i)
			}
		}
	}
	return n
}

func getNumberWithinCoordinates(input []string, x int, y int) (int, bool) {
	if y < 0 || y >= len(input) {
		return 0, false
	}
	if x < 0 || x >= len(input[0]) {
		return 0, false
	}

	if unicode.IsNumber(rune(input[y][x])) {
		buffer := getNumbersLeft(input[y], x-1) + getNumbersRight(input[y], x)
		i, _ := strconv.Atoi(buffer)
		return i, true
	}
	
	return 0, false
}

func getNumbersLeft(s string, x int) string {
	if x < 0 || !unicode.IsNumber(rune(s[x])) {
		return ""
	}

	return getNumbersLeft(s, x-1) + string(s[x])
}

func getNumbersRight(s string, x int) string {
	if x >= len(s) || !unicode.IsNumber(rune(s[x])) {
		return ""
	}

	return string(s[x]) + getNumbersRight(s, x+1)
}
