package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"time"
)

// https://adventofcode.com/2016/day/7
func main() {
	addresses, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(addresses)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(addresses)
	fmt.Println("2. Result: ", res2)
}

func solve_1(addresses []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	var res int

	for _, ip := range addresses {

		chars := make(map[int]string)

		isFound := false
		isInvalid := false
		isInBrackets := false

		for pos, letter := range ip {
			chars[pos] = string(letter)

			// Проверка если внутри []
			if letter == '[' {
				isInBrackets = true
			} else if letter == ']' {
				isInBrackets = false
			}

			if pos >= 3 {

				// проверка на сочетание типа: "abba"
				if chars[pos-3] == chars[pos] && chars[pos-1] == chars[pos-2] && chars[pos] != chars[pos-1] {

					isFound = true

					// Если в [] - ip не валидный
					if isInBrackets {
						isInvalid = true
					}
				}
			}
		}

		if !isInvalid && isFound {
			res += 1
		}
	}

	return res
}

func solve_2(addresses []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	var res int

	for _, ip := range addresses {

		// снаружи [] (supernet)
		supernetChars := make(map[int]string)

		// внутри [] (hypernet)
		hypernetChars := make(map[int]string)

		isInBrackets := false

		for pos, letter := range ip {
			// Проверка если внутри []
			if letter == '[' {
				isInBrackets = true
			} else if letter == ']' {
				isInBrackets = false
			}

			if isInBrackets {
				hypernetChars[pos] = string(letter)
			} else {
				supernetChars[pos] = string(letter)
			}
		}

	supernetLoop:
		for pos, _ := range supernetChars {

			if pos > 1 {
				// проверка на ABA
				if supernetChars[pos] == supernetChars[pos-2] && supernetChars[pos] != supernetChars[pos-1] {

					for hPos, _ := range hypernetChars {

						// проверка BAB
						if hypernetChars[hPos] == hypernetChars[hPos-2] &&
							hypernetChars[hPos] != hypernetChars[hPos-1] &&
							// проверка что BAB = ABA
							hypernetChars[hPos] == supernetChars[pos-1] &&
							hypernetChars[hPos-1] == supernetChars[pos] &&
							hypernetChars[hPos-1] == supernetChars[pos-2] {

							res += 1
							break supernetLoop
						}

					}

				}
			}
		}
	}

	return res
}
