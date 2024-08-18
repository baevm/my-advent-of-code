package main

import (
	"2016/helpers"
	"fmt"
	"time"
)

// https://adventofcode.com/2016/day/19
func main() {
	puzzleInput := 3004953

	res1 := solve_1(puzzleInput)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(puzzleInput)
	fmt.Println("2. Result: ", res2)
}

func solve_1(puzzleInput int) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	// elfId: presentsCount
	elfs := make(map[int]int, puzzleInput)

	for i := 1; i <= puzzleInput; i++ {
		elfs[i] = 1
	}

	curr := 1

	for {
		next := getNextStep(elfs, curr, puzzleInput)

		if elfs[curr] > 0 && elfs[next] > 0 {
			// take all presents of next elf
			elfs[curr] += elfs[next]
			elfs[next] = 0
		}

		if elfs[curr] == puzzleInput {
			return curr
		}

		curr = next

	}

}

func solve_2(puzzleInput int) int {
	defer helpers.TimeTrack(time.Now(), "Task 2")

	i := 1

	for i*3 < puzzleInput {
		i *= 3
	}

	return puzzleInput - i
}

type Node struct {
	Val  int
	Next *Node
}

func getNextStep(elfs map[int]int, curr int, max int) int {
	next := curr

	if curr+1 > max {
		next = 1
	} else {
		next += 1

		for elfs[next] == 0 {
			if next+1 > max {
				next = 1
			} else {
				next += 1
			}
		}
	}

	return next
}
