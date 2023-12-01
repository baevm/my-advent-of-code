package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"
)

// https://adventofcode.com/2023/day/1
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
	var res = 0
	regex := regexp.MustCompile("[0-9]+")

	for _, line := range lines {
		foundNums := regex.FindAllString(line, -1)

		// firstDigit digit of firstDigit str
		firstDigit := foundNums[0][0]

		// last digit of second str
		lastStr := foundNums[len(foundNums)-1]
		lastDigit := lastStr[len(lastStr)-1]

		newNumStr := string(firstDigit) + string(lastDigit)

		newNum, err := strconv.Atoi(newNumStr)

		if err != nil {
			return 0, err
		}

		res += newNum
	}

	return res, nil
}

func solve_2(lines []string) (int, error) {
	var res = 0

	regex := regexp.MustCompile(`(?i:(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|[0-9])`)
	digitLetters := map[string]string{
		"one":   "1",
		"two":   "2",
		"three": "3",
		"four":  "4",
		"five":  "5",
		"six":   "6",
		"seven": "7",
		"eight": "8",
		"nine":  "9",
	}

	for _, line := range lines {
		// retarded solution
		// oneight == 83 etc...
		line = strings.Replace(line, "oneight", "oneeight", -1)
		line = strings.Replace(line, "threeight", "threeeight", -1)
		line = strings.Replace(line, "fiveight", "fiveeight", -1)
		line = strings.Replace(line, "nineight", "nineeight", -1)
		line = strings.Replace(line, "twone", "twoone", -1)
		line = strings.Replace(line, "sevenine", "sevennine", -1)
		line = strings.Replace(line, "eightwo", "eighttwo", -1)

		foundDigits := regex.FindAllString(line, -1)

		firstStr := foundDigits[0]
		firstDigit := ""

		if _, isExist := digitLetters[firstStr]; isExist {
			firstDigit = digitLetters[firstStr]
		} else {
			firstDigit = string(firstStr[0])
		}

		lastStr := foundDigits[len(foundDigits)-1]
		lastDigit := ""

		if _, isExist := digitLetters[lastStr]; isExist {
			lastDigit = digitLetters[lastStr]
		} else {
			lastDigit = string(lastStr[len(lastStr)-1])
		}

		newNumStr := firstDigit + lastDigit

		newNum, err := strconv.Atoi(newNumStr)

		if err != nil {
			return 0, err
		}

		res += newNum
	}

	return res, nil
}
