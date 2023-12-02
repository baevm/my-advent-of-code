package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"strconv"
	"strings"
)

// https://adventofcode.com/2023/day/2
func main() {
	lines, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1, err := solve_1(lines)

	if err != nil {
		log.Fatalln(err)
	}

	fmt.Println("1. Result: ", res1)

	res2, err := solve_2(lines)

	if err != nil {
		log.Fatalln(err)
	}

	fmt.Println("2. Result: ", res2)
}

func solve_1(lines []string) (int, error) {
	res := 0

	maxAmounts := map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}

	for _, line := range lines {
		game := strings.Split(line, ":") // ["Game 52", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]

		gameId := getGameId(game)

		res += gameId // add gameId now. l8 if gameset is invalid, substract current gameId

		gameSets := strings.Split(game[1], ";") // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]

		// Go through gamesets
		// If we find bad gameset => game is not valid
	gameLoop:
		for _, set := range gameSets {
			cubes := strings.Split(set, ", ") // ["3 blue", "4 red"]

			// Go through cubes in 1 set
			for _, cube := range cubes {
				amountAndCubeColor := strings.Fields(cube) // ["3", "blue"]

				amountStr := amountAndCubeColor[0]
				cubeColor := amountAndCubeColor[1]

				amount, err := strconv.Atoi(amountStr)

				if err != nil {
					return 0, err
				}

				if maxAmount, isExist := maxAmounts[cubeColor]; isExist && amount > maxAmount {
					res -= gameId
					break gameLoop
				}
			}
		}

	}

	return res, nil
}

func solve_2(lines []string) (int, error) {
	res := 0

	for _, line := range lines {
		maxAmounts := map[string]int{
			"red":   0,
			"green": 0,
			"blue":  0,
		}

		game := strings.Split(line, ":") // ["Game 52", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]

		gameSets := strings.Split(game[1], ";") // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]

		// Go through gamesets
		// If we find bad gameset => game is not valid

		for _, set := range gameSets {
			cubes := strings.Split(set, ", ") // ["3 blue", "4 red"]

			// Go through cubes in 1 set
			for _, cube := range cubes {
				amountAndCubeColor := strings.Fields(cube) // ["3", "blue"]

				amountStr := amountAndCubeColor[0]
				cubeColor := amountAndCubeColor[1]

				amount, err := strconv.Atoi(amountStr)

				if err != nil {
					return 0, err
				}

				maxAmounts[cubeColor] = max(maxAmounts[cubeColor], amount)
			}
		}

		powerSet := 1

		for _, v := range maxAmounts {
			powerSet *= v
		}

		res += powerSet
	}

	return res, nil
}

func max(num1 int, num2 int) int {
	if num1 > num2 {
		return num1
	}

	return num2
}

func getGameId(game []string) int {
	gameName := strings.Fields(game[0]) // ["Game", "52"]
	gameIdStr := gameName[1]            // "52"

	gameId, err := strconv.Atoi(gameIdStr)

	if err != nil {
		panic(err)
	}

	return gameId
}
