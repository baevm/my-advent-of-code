package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/1
func main() {
	instruction_lines, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(instruction_lines)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(instruction_lines)
	fmt.Println("2. Result: ", res2)
}

func solve_1(instruction_lines []string) string {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	keypad := [3][3]string{
		{"1", "2", "3"},
		{"4", "5", "6"},
		{"7", "8", "9"},
	}

	const (
		MIN_Y = 0
		MAX_Y = 2
		MIN_X = 0
		MAX_X = 2
	)

	code := strings.Builder{}

	// Start at "5"
	pos := Point{x: 1, y: 1}

	for _, line := range instruction_lines {

		for _, dir := range line {
			switch dir {
			case 'U':
				if pos.y > MIN_Y {
					pos.y -= 1
				}
			case 'D':
				if pos.y < MAX_Y {
					pos.y += 1
				}
			case 'R':
				if pos.x < MAX_X {
					pos.x += 1
				}
			case 'L':
				if pos.x > MIN_X {
					pos.x -= 1
				}
			}
		}

		code.WriteString(keypad[pos.y][pos.x])
	}

	return code.String()
}

func solve_2(instruction_lines []string) string {
	defer helpers.TimeTrack(time.Now(), "Task 2")

	const (
		MIN_Y = 0
		MAX_Y = 4
		MIN_X = 0
		MAX_X = 4
	)

	keypad := [5][5]string{
		{"-", "-", "1", "-", "-"},
		{"-", "2", "3", "4", "-"},
		{"5", "6", "7", "8", "9"},
		{"-", "A", "B", "C", "-"},
		{"-", "-", "D", "-", "-"},
	}

	code := strings.Builder{}

	// Start at "5"
	pos := Point{x: 0, y: 2}

	for _, line := range instruction_lines {

		for _, dir := range line {
			switch dir {
			case 'U':
				if pos.y > MIN_Y && keypad[pos.y-1][pos.x] != "-" {
					pos.y -= 1
				}
			case 'D':
				if pos.y < MAX_X && keypad[pos.y+1][pos.x] != "-" {
					pos.y += 1
				}
			case 'R':
				if pos.x < MAX_X && keypad[pos.y][pos.x+1] != "-" {
					pos.x += 1
				}
			case 'L':
				if pos.x > MIN_X && keypad[pos.y][pos.x-1] != "-" {
					pos.x -= 1
				}
			}
		}

		code.WriteString(keypad[pos.y][pos.x])
	}

	return code.String()
}

type Point struct {
	x, y int
}
