package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/12
func main() {
	puzzle, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(puzzle)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(puzzle)
	fmt.Println("2. Result: ", res2)
}

func solve_1(instructions []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	registers := make(map[byte]int)

	calculate(registers, instructions)

	return registers[byte('a')]
}

func solve_2(instructions []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 2")

	registers := make(map[byte]int)
	registers[byte('c')] = 1

	calculate(registers, instructions)

	return registers[byte('a')]
}

func calculate(registers map[byte]int, instructions []string) {
	i := 0

	for i < len(instructions) {
		inst := instructions[i]

		switch inst[0:3] {
		case "cpy":
			fields := strings.Fields(inst)

			first := fields[1]
			second := fields[len(fields)-1]

			num, err := strconv.Atoi(first)

			if err != nil {
				// cpy a c
				registers[second[0]] = registers[first[0]]
			} else {
				// cpy 1 a
				registers[second[0]] = num
			}

			i += 1
		case "inc":
			reg := inst[len(inst)-1]
			registers[reg] += 1

			i += 1
		case "dec":
			reg := inst[len(inst)-1]
			registers[reg] -= 1

			i += 1
		case "jnz":
			fields := strings.Fields(inst)

			regOrInt := fields[1]

			numToSkipStr := fields[2]
			numToSkip, _ := strconv.Atoi(numToSkipStr)

			regAsInt, err := strconv.Atoi(regOrInt)

			if err != nil {
				// jnz c 2
				if registers[regOrInt[0]] > 0 {
					i += numToSkip
				} else {
					i += 1
				}

			} else {
				// jnz 1 5
				if regAsInt > 0 {
					i += numToSkip
				} else {
					i += 1
				}
			}

		}
	}
}
