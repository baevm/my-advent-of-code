package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/9
func main() {
	puzzle, err := helpers.OpenTaskAsString("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(puzzle)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(puzzle)
	fmt.Println("2. Result: ", res2)
}

func solve_1(puzzle string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	var sb strings.Builder

	i := 0

	// "A(1x5)BC"
	for i < len(puzzle) {
		ch := puzzle[i]

		if ch == '(' {
			// e.g. 1x5
			multiplStr := ""

			// skip '('
			i += 1

			for puzzle[i] != ')' {
				multiplStr += string(puzzle[i])
				i += 1
			}

			repeater := strings.Split(multiplStr, "x")

			// next 1 char
			charsToRepeat, _ := strconv.Atoi(repeater[0])

			// repeat 5 times
			repeatTimes, _ := strconv.Atoi(repeater[1])

			// "B"
			strToRepeat := ""

			for charsToRepeat > 0 {
				i += 1
				charsToRepeat -= 1

				strToRepeat += string(puzzle[i])
			}

			// "BBBBB"
			repeatedStr := strings.Repeat(strToRepeat, repeatTimes)

			sb.WriteString(repeatedStr)
		} else {
			sb.WriteByte(ch)
		}

		i += 1
	}

	return sb.Len()
}

func solve_2(puzzle string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	var sb strings.Builder

	return sb.Len()
}
