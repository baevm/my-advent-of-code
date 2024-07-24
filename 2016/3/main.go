package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/3
func main() {
	trianlge_lines, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(trianlge_lines)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(trianlge_lines)
	fmt.Println("2. Result: ", res2)
}

func solve_1(trianlge_lines []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	var res int

	reg := regexp.MustCompile(`\s\s+`)

	nums := make([]int, 3)

	for _, line := range trianlge_lines {
		cleanNumStr := strings.TrimSpace(reg.ReplaceAllString(line, " "))
		strNums := strings.Split(cleanNumStr, " ")

		for i, strNum := range strNums {
			num, _ := strconv.Atoi(strNum)
			nums[i] = num
		}

		if isValidTriangle(nums[0], nums[1], nums[2]) {
			res += 1
		}
	}

	return res
}

func solve_2(trianlge_lines []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 2")

	var res int

	// Числа сохраняются вертикально, вместо горизонтального из первой части
	// 101 301 501
	//  |   |   |
	// 102 302 502
	//  |   |   |
	// 103 303 503
	testArr := [3][3]int{
		{0, 0, 0},
		{0, 0, 0},
		{0, 0, 0},
	}

	currentLine := 0

	reg := regexp.MustCompile(`\s\s+`)

	for _, line := range trianlge_lines {
		cleanNumStr := strings.TrimSpace(reg.ReplaceAllString(line, " "))
		strNums := strings.Split(cleanNumStr, " ")

		for i, strNum := range strNums {
			num, _ := strconv.Atoi(strNum)

			testArr[i][currentLine] = num
		}

		if currentLine == 2 {
			if isValidTriangle(testArr[0][0], testArr[0][1], testArr[0][2]) {
				res += 1
			}

			if isValidTriangle(testArr[1][0], testArr[1][1], testArr[1][2]) {
				res += 1
			}

			if isValidTriangle(testArr[2][0], testArr[2][1], testArr[2][2]) {
				res += 1
			}

			testArr[0][0] = 0
			testArr[0][1] = 0
			testArr[0][2] = 0

			testArr[1][0] = 0
			testArr[1][1] = 0
			testArr[1][2] = 0

			testArr[2][0] = 0
			testArr[2][1] = 0
			testArr[2][2] = 0

			currentLine = 0
		} else {
			currentLine += 1
		}
	}

	return res
}

func isValidTriangle(num1 int, num2 int, num3 int) bool {
	if num1+num2 > num3 && num2+num3 > num1 && num1+num3 > num2 {
		return true
	}

	return false
}
