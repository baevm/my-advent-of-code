package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/4
func main() {
	room_names, err := helpers.OpenTask("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(room_names)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(room_names)
	fmt.Println("2. Result: ", res2)
}

func solve_1(roomNames []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	var res int

	checksumRegex := regexp.MustCompile(`[a-z]+`)
	sectorIdRegex := regexp.MustCompile(`[0-9]+`)

	for _, roomName := range roomNames {
		// totally-real-room-200[decoy]
		lastDashIdx := strings.LastIndex(roomName, "-")

		// totally-real-room
		encryptedName := roomName[:lastDashIdx]

		// 200[decoy]
		sectorIdWithChecksum := roomName[lastDashIdx+1:]

		// счетчик символов
		nameHmap := make(map[rune]int)

		// массив с most common символами по убыванию
		nameLetters := make([]rune, 0)

		for _, ch := range encryptedName {
			if ch == '-' {
				continue
			}

			if val, isExist := nameHmap[ch]; isExist {
				nameHmap[ch] = val + 1
			} else {
				nameHmap[ch] = 1
				nameLetters = append(nameLetters, ch)
			}
		}

		slices.SortFunc(nameLetters, func(a rune, b rune) int {
			// если число встреч символов в строке одинаковое -
			// сортируем по алфавиту
			if nameHmap[b] == nameHmap[a] {
				return int(a - b)
			}

			// сортируем по количеству встреч символов в строке
			return nameHmap[b] - nameHmap[a]
		})

		// 200
		sectorId := sectorIdRegex.FindString(sectorIdWithChecksum)

		// decoy
		checksum := checksumRegex.FindString(sectorIdWithChecksum)

		isValidChecksum := true

		for i, ch := range checksum {
			// если символа из checkSum не существует - имя не валидное
			if _, isExist := nameHmap[ch]; !isExist {
				isValidChecksum = false
				break
			}

			// если символ не равен символу по i самому встречаемому
			if nameLetters[i] != ch {
				isValidChecksum = false
			}
		}

		if isValidChecksum {
			sectorIdNum, _ := strconv.Atoi(sectorId)
			res += sectorIdNum
		}
	}

	return res
}

func solve_2(roomNames []string) int {
	defer helpers.TimeTrack(time.Now(), "Task 2")

	var res int

	minCharCode := 97

	for _, roomName := range roomNames {
		// totally-real-room-200[decoy]
		endOfNameIdx := strings.Index(roomName, "[")

		// totally-real-room-200
		encryptedNameWithSector := roomName[:endOfNameIdx]

		startOfSectorIdx := strings.LastIndex(encryptedNameWithSector, "-")

		// totally-real-room
		encryptedName := encryptedNameWithSector[:startOfSectorIdx]

		// 200
		sectorIdStr := encryptedNameWithSector[startOfSectorIdx+1:]

		sectorId, _ := strconv.Atoi(sectorIdStr)

		encryptedNameSlice := make([]string, len(encryptedName))

		for i, ch := range encryptedName {
			if ch == '-' {
				encryptedNameSlice[i] = "-"
				continue
			}

			newCharCode := minCharCode + (int(ch)-minCharCode+sectorId)%26

			s := fmt.Sprintf("%c", newCharCode)
			encryptedNameSlice[i] = s
		}

		decryptedName := strings.Join(encryptedNameSlice, "")

		if strings.Contains(decryptedName, "north") {
			return sectorId
		}
	}

	return res
}
