package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Race struct {
	time     int
	distance int
}

// https://adventofcode.com/2023/day/6
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

func getTimesDistances(lines []string) (times []string, distances []string) {
	timesLine := strings.Split(lines[0], ":")
	times = strings.Fields(timesLine[1])

	distancesLine := strings.Split(lines[1], ":")
	distances = strings.Fields(distancesLine[1])

	return
}

func solve_1(lines []string) int {
	res := 1

	// parsing
	times, distances := getTimesDistances(lines)

	races := make(map[int]Race, len(times))

	for i := 0; i < len(times); i++ {
		time, _ := strconv.Atoi(times[i])
		distance, _ := strconv.Atoi(distances[i])

		races[i] = Race{
			time:     time,
			distance: distance,
		}
	}

	// actually calculating
	for _, race := range races {
		options := 0

		for i := 1; i <= race.time; i++ {
			traveled := i * (race.time - i)

			if traveled > race.distance {
				options += 1
			}
		}

		res *= options
	}

	return res
}

func solve_2(lines []string) int {
	res := 0

	// parsing
	times, distances := getTimesDistances(lines)

	realTimeStr := strings.Join(times, "")
	realDistanceStr := strings.Join(distances, "")

	realTime, _ := strconv.Atoi(realTimeStr)
	realDistance, _ := strconv.Atoi(realDistanceStr)

	for i := 0; i <= realTime; i++ {
		traveled := i * (realTime - i)

		if traveled > realDistance {
			res += 1
		}
	}

	return res
}
