package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"strconv"
	"strings"
)

// https://adventofcode.com/2023/day/5
func main() {
	lines, err := helpers.OpenTaskAsString("./task.txt")

	if err != nil {
		log.Fatalln(err)
	}

	res1 := solve_1(lines)

	fmt.Println("1. Result: ", res1)

	res2 := solve_2(lines)

	fmt.Println("2. Result: ", res2)
}

func solve_1(lines string) int {
	res := 0

	seeds := []int{}
	maps := make(map[string][][]int)

	// build maps
	for _, line := range strings.Split(lines, "\n\n") {
		// extract seeds from first line
		if strings.Contains(line, "seeds: ") {
			seeds = getInts(strings.ReplaceAll(line, "seeds: ", ""))
			continue
		}

		mappings := strings.Split(line, "\n")

		name := strings.Split(mappings[0], " ")[0]

		for i := 1; i < len(mappings); i++ {
			maps[name] = append(maps[name], getInts(mappings[i]))
		}
	}

	// calculate
	for _, seed := range seeds {
		ts := score_1(seed, maps)

		if res == 0 {
			res = ts
			continue
		}

		res = min(ts, res)
	}

	return res
}

func solve_2(lines string) int {
	res := -1

	seeds := []int{}
	maps := make(map[string][][]int)

	// build maps
	for _, line := range strings.Split(lines, "\n\n") {
		// extract seeds from first line
		if strings.Contains(line, "seeds: ") {
			seeds = getInts(strings.ReplaceAll(line, "seeds: ", ""))
			continue
		}

		mappings := strings.Split(line, "\n")

		name := strings.Split(mappings[0], " ")[0]

		for i := 1; i < len(mappings); i++ {
			maps[name] = append(maps[name], getInts(mappings[i]))
		}
	}

	// calculate
	for i := 0; i < len(seeds); i += 2 {
		for j := seeds[i]; j < seeds[i]+seeds[i+1]; j++ {
			ts := score_2(j, maps)

			if res == -1 {
				res = ts
				continue
			}

			res = min(ts, res)
		}
	}

	return res
}

func getInts(line string) []int {
	res := []int{}

	for _, seed := range strings.Split(line, " ") {
		seedInt, err := strconv.Atoi(seed)

		if err != nil {
			panic(err)
		}

		res = append(res, seedInt)
	}

	return res
}

func score_1(seed int, maps map[string][][]int) int {
	soil := getDest_1(seed, maps[`seed-to-soil`])
	fertilizer := getDest_1(soil, maps[`soil-to-fertilizer`])
	water := getDest_1(fertilizer, maps[`fertilizer-to-water`])
	light := getDest_1(water, maps[`water-to-light`])
	temp := getDest_1(light, maps[`light-to-temperature`])
	humidity := getDest_1(temp, maps[`temperature-to-humidity`])
	location := getDest_1(humidity, maps[`humidity-to-location`])

	return location
}

func getDest_1(seed int, mapping [][]int) int {
	for _, m := range mapping {
		if m[1] <= seed && seed <= m[1]+m[2] {
			return m[0] + (seed - m[1])
		}
	}

	return seed
}

func score_2(seed int, maps map[string][][]int) int {
	soil := getDest_2(seed, maps[`seed-to-soil`])
	fertilizer := getDest_2(soil, maps[`soil-to-fertilizer`])
	water := getDest_2(fertilizer, maps[`fertilizer-to-water`])
	light := getDest_2(water, maps[`water-to-light`])
	temp := getDest_2(light, maps[`light-to-temperature`])
	humidity := getDest_2(temp, maps[`temperature-to-humidity`])
	location := getDest_2(humidity, maps[`humidity-to-location`])

	return location
}

func getDest_2(seed int, mapping [][]int) int {
	for _, m := range mapping {
		if m[1] <= seed && seed < m[1]+m[2] {
			return m[0] + (seed - m[1])
		}
	}

	return seed
}
