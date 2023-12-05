package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"math"
	"strconv"
	"strings"
)

// https://adventofcode.com/2023/day/4
func main() {
	lines, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1, res2 := solve_1(lines)

	fmt.Println("1. Result: ", res1)

	fmt.Println("2. Result: ", res2)
}

func solve_1(lines []string) (int, int) {
	res1 := 0

	cards := make([]int, len(lines))

	for i := range cards {
		cards[i] = 1
	}

	for lineIdx, line := range lines {
		parts := strings.Split(line, " | ")

		winningNumsStr := strings.Split(parts[0], ": ")[1]

		winningNums := strings.Fields(winningNumsStr)
		myNums := strings.Fields(parts[1])

		winningNumsSet := make(map[int]struct{}, len(winningNums))

		for _, numStr := range winningNums {
			num, err := strconv.Atoi(numStr)

			if err != nil {
				panic(err)
			}

			winningNumsSet[num] = struct{}{}
		}

		winPoints := 0

		for _, numStr := range myNums {
			num, err := strconv.Atoi(numStr)

			if err != nil {
				panic(err)
			}

			if _, isExist := winningNumsSet[num]; !isExist {
				continue
			}

			winPoints += 1
		}

		if winPoints > 0 {
			res1 += int(math.Pow(2, float64(winPoints-1)))
		}

		for i := 0; i < winPoints; i++ {
			cards[lineIdx+i+1] += cards[lineIdx]
		}
	}

	res2 := 0

	for _, v := range cards {
		res2 += v
	}

	return res1, res2
}

func solve_2(lines []string) int {
	res := 0

	return res
}
