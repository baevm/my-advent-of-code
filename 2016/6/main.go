package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/6
func main() {
	messages, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(messages)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(messages)
	fmt.Println("2. Result: ", res2)
}

const ENGLISH = "abcdefghijklmnopqrstuvwxyz"

func solve_1(messages []string) string {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	/*
		{
			0: {a: 0, b: 0 ...}
			1: {a: 0, b: 1...}
		}
	*/
	positionCounter := make(map[int]map[string]int)

	wordLen := len(messages[0])

	/* Initialize map with letter counter for every position */
	for i := 0; i < wordLen; i++ {
		positionCounter[i] = make(map[string]int)

		for j := 0; j < len(ENGLISH); j++ {
			letter := string(ENGLISH[j])
			positionCounter[i][letter] = 0
		}
	}

	/* Process input  */
	for _, message := range messages {
		letters := strings.Split(message, "")

		for idx, letter := range letters {
			positionCounter[idx][letter] += 1
		}
	}

	resultString := make([]string, wordLen)

	/* Calculate result */
	for position, counter := range positionCounter {

		// find MAX in map of letters for position
		maxCount := 0
		letterWithMax := ""

		for letter, count := range counter {
			if count > maxCount {
				maxCount = count
				letterWithMax = letter
			}
		}

		resultString[position] = letterWithMax
	}

	return strings.Join(resultString, "")
}

func solve_2(messages []string) string {
	defer helpers.TimeTrack(time.Now(), "Task 2")

	/*
		{
			0: {a: 0, b: 0 ...}
			1: {a: 0, b: 1...}
		}
	*/
	positionCounter := make(map[int]map[string]int)

	wordLen := len(messages[0])

	// Initialize map with letter counter for every position
	for i := 0; i < wordLen; i++ {
		positionCounter[i] = make(map[string]int)

		for j := 0; j < len(ENGLISH); j++ {
			letter := string(ENGLISH[j])
			positionCounter[i][letter] = 0
		}
	}

	/* Process input  */
	for _, message := range messages {
		letters := strings.Split(message, "")

		for idx, letter := range letters {
			positionCounter[idx][letter] += 1
		}
	}

	resultString := make([]string, wordLen)

	/* Calculate result */
	for position, counter := range positionCounter {

		// find MIN in map of letters for position
		minCount := 999
		letterWithMin := ""

		for letter, count := range counter {
			if count != 0 && count < minCount {
				minCount = count
				letterWithMin = letter
			}
		}

		resultString[position] = letterWithMin
	}

	return strings.Join(resultString, "")
}
