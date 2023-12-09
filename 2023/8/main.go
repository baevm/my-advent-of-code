package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"strings"
)

// https://adventofcode.com/2023/day/8
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

func solve_1(lines []string) int {
	steps := 0

	// navigation defined on line 0
	navigation := strings.Split(lines[0], "")

	network := make(map[string][]string)

	// nodes start from line 2
	for i := 2; i < len(lines); i++ {
		line := lines[i]
		nodeWithSteps := strings.Split(line, " = ")

		node := nodeWithSteps[0]
		stepsWithBrackets := nodeWithSteps[1]

		// clear brackets
		stepsStr := stepsWithBrackets[1 : len(stepsWithBrackets)-1]
		steps := strings.Split(stepsStr, ", ")

		network[node] = steps
	}

	curr := "AAA"

	i := 0
	for curr != "ZZZ" {
		next_steps := network[curr]

		nav := navigation[i]

		if nav == "L" {
			curr = next_steps[0]
		} else {
			curr = next_steps[1]
		}

		steps += 1

		// if we reach end of navigation array
		// start from begining
		if i == len(navigation)-1 {
			i = 0
		} else {
			i += 1
		}
	}

	return steps
}

func solve_2(lines []string) int {
	steps := 1

	// navigation defined on line 0
	navigation := strings.Split(lines[0], "")

	network := make(map[string][]string)

	// nodes start from line 2
	for i := 2; i < len(lines); i++ {
		line := lines[i]
		nodeWithSteps := strings.Split(line, " = ")

		node := nodeWithSteps[0]
		stepsWithBrackets := nodeWithSteps[1]

		// clear brackets
		stepsStr := stepsWithBrackets[1 : len(stepsWithBrackets)-1]
		steps := strings.Split(stepsStr, ", ")

		network[node] = steps
	}

	for key := range network {
		if string(key[len(key)-1]) == "A" {
			j := getZIndex(navigation, key, network)
			steps = LCM(steps, j)
		}
	}

	return steps
}

func getZIndex(navigation []string, key string, network map[string][]string) int {
	pos := key
	idx := 0

	for string(pos[len(pos)-1]) != "Z" {
		d := navigation[idx%len(navigation)]

		if d == "L" {
			pos = network[pos][0]
		} else {
			pos = network[pos][1]
		}

		idx += 1
	}

	return idx
}

func GCD(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

func LCM(a, b int) int {
	result := a * b / GCD(a, b)

	return result
}
