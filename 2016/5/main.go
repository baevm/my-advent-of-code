package main

import (
	"2016/helpers"
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"log"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/5
func main() {
	puzzle, err := helpers.OpenTaskAsString("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(puzzle)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(puzzle)
	fmt.Println("2. Result: ", res2)
}

func solve_1(puzzle string) string {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	pb := strings.Builder{}
	idx := 0

	for pb.Len() != 8 {
		hash := md5.Sum([]byte(puzzle + strconv.Itoa(idx)))

		hashStr := hex.EncodeToString(hash[:])

		// if hex starts with "00000"
		// get char at index 5 and append to password
		if hashStr[0:5] == "00000" {
			pb.WriteByte(hashStr[5])
		}

		idx += 1

	}

	return pb.String()
}

func solve_2(puzzle string) string {
	defer helpers.TimeTrack(time.Now(), "Task 2")

	password := make([]string, 8)
	foundChars := 0

	idx := 0

	for foundChars != 8 {
		hash := md5.Sum([]byte(puzzle + strconv.Itoa(idx)))

		hashStr := hex.EncodeToString(hash[:])

		// if hex starts with "00000"
		// get int at index 5
		// check if this position is valid
		// and add char from index 6 to password at given position
		if hashStr[0:5] == "00000" {
			posStr := string(hashStr[5])
			char := string(hashStr[6])

			pos, err := strconv.Atoi(posStr)

			if pos < 8 && password[pos] == "" && err == nil {
				password[pos] = char
				foundChars += 1
			}
		}

		idx += 1
	}

	return strings.Join(password, "")
}
