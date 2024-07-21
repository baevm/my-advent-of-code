package main

import (
	"2016/helpers"
	"fmt"
	"log"
	"math"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2016/day/1
func main() {
	document, err := helpers.OpenTaskAsString("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(document)
	fmt.Println("1. Result: ", res1)

	res2 := solve_2(document)
	fmt.Println("2. Result: ", res2)
}

func solve_1(document string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	directions := strings.Split(document, ", ")

	// Start facing North
	// with coords: (0,0)
	currDir := N
	coord := Point{
		x: 0,
		y: 0,
	}

	for _, dir := range directions {
		blockNum, _ := strconv.Atoi(dir[1:])

		switch dir[0] {
		// North - East - South - West - North
		case 'R':
			switch currDir {
			case N:
				coord.x += blockNum
				currDir = E
			case E:
				coord.y -= blockNum
				currDir = S
			case S:
				coord.x -= blockNum
				currDir = W
			case W:
				coord.y += blockNum
				currDir = N
			}

		// North - West - South - East - North
		case 'L':
			switch currDir {
			case N:
				coord.x -= blockNum
				currDir = W
			case W:
				coord.y -= blockNum
				currDir = S
			case S:
				coord.x += blockNum
				currDir = E
			case E:
				coord.y += blockNum
				currDir = N
			}
		}
	}

	blocksAway := math.Abs(float64(coord.x)) + math.Abs(float64(coord.y))

	return int(blocksAway)
}

func solve_2(document string) int {
	defer helpers.TimeTrack(time.Now(), "Task 1")

	directions := strings.Split(document, ", ")

	compass := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}

	currDir := 0
	coord := Point{
		x: 0,
		y: 0,
	}

	visited := make(map[Point]bool)
	visited[coord] = true

	for _, dir := range directions {
		blockNum, _ := strconv.Atoi(dir[1:])

		// R
		pos := 1

		if dir[0] == 'L' {
			// L
			pos = -1
		}

		currDir = (currDir + (pos)) % len(compass)

		for i := 0; i < blockNum; i++ {
			coord.x += getElementFromSlice(compass, currDir)[0]
			coord.y += getElementFromSlice(compass, currDir)[1]

			if _, isExist := visited[coord]; isExist {
				blocksAway := math.Abs(float64(coord.x)) + math.Abs(float64(coord.y))
				return int(blocksAway)
			} else {
				visited[coord] = true
			}
		}
	}

	return 0
}

type Point struct {
	x, y int
}

const (
	N = iota + 1
	E
	S
	W
)

func getElementFromSlice[T any](array []T, index int) T {
	if index < 0 {
		return array[len(array)-helpers.Abs(index)]
	}

	return array[index]
}
