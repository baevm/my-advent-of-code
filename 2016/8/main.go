package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/8
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

	result, _ := solve(puzzle)

	return result
}

func solve_2(puzzle string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	_, screen := solve(puzzle)

	fmt.Println(display(screen))

	return -1
}

func solve(puzzle string) (int, [][]string) {
	// 50px wide, 6px tall
	screen := make([][]string, 6)

	for i := range screen {
		screen[i] = make([]string, 50)

		for j := range screen[i] {
			screen[i][j] = "."
		}
	}

	lines := strings.Split(puzzle, "\n")

	for _, line := range lines {
		parts := strings.Split(line, " ")

		switch parts[0] {
		case "rect":
			// 1x1
			size := strings.Split(parts[1], "x")

			cols, _ := strconv.Atoi(size[0])
			rows, _ := strconv.Atoi(size[1])

			for i := range rows {
				for j := range cols {
					screen[i][j] = "#"
				}
			}

		case "rotate":
			// row | column
			direction := parts[1]

			// y=0 | x=0
			target, _ := strconv.Atoi(strings.Split(parts[2], "=")[1])

			amount, _ := strconv.Atoi(parts[4])

			switch direction {
			case "row":
				newRow := rotateRight(screen[target], amount)
				screen[target] = newRow

			case "column":
				prevColumn := make([]string, 0)

				for i := range len(screen) {
					prevColumn = append(prevColumn, screen[i][target])
				}

				newColumn := rotateRight(prevColumn, amount)

				for i := range screen {
					screen[i][target] = newColumn[i]
				}
			}
		}
	}

	result := 0

	for _, line := range screen {
		for _, elem := range line {
			if elem == "#" {
				result += 1
			}
		}
	}

	return result, screen
}

func display(screen [][]string) string {
	var sb strings.Builder

	for _, row := range screen {
		sb.WriteString(strings.Join(row, ""))
		sb.WriteString("\n")
	}

	return sb.String()
}

func rotateRight(row []string, amount int) []string {
	n := len(row)
	amount = amount % n
	result := make([]string, n)

	for i := range row {
		result[(i+amount)%n] = row[i]
	}

	return result
}
