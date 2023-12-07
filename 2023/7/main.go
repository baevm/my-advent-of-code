package main

import (
	"2023/helpers"
	"fmt"
	"log"
	"sort"
	"strconv"
	"strings"
)

// https://adventofcode.com/2023/day/7
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

type Hand struct {
	hand     string
	bid      int
}

// 0 - high card -- weakest
// 1 - one pair
// 2 - two pair
// 3 - three of a kind
// 4 - full house
// 5 - four of a kind
// 6 - five of a kind -- strongest
const (
	HIGH_CARD = iota
	ONE_PAIR
	TWO_PAIR
	THREE_OF_KIND
	FULL_HOUSE
	FOUR_OF_KIND
	FIVE_OF_KIND
)

// part 1 scores
var scores_1 = map[rune]int{
	'A': 13,
	'K': 12,
	'Q': 11,
	'J': 10,
	'T': 9,
	'9': 8,
	'8': 7,
	'7': 6,
	'6': 5,
	'5': 4,
	'4': 3,
	'3': 2,
	'2': 1,
}

// part 2 scores
var scores_2 = map[rune]int{
	'A': 13,
	'K': 12,
	'Q': 11,
	'T': 10,
	'9': 9,
	'8': 8,
	'7': 7,
	'6': 6,
	'5': 5,
	'4': 4,
	'3': 3,
	'2': 2,
	'J': 1,
}

func solve_1(lines []string) int {
	res := 0

	// index = iota of card type
	hands := make([][]Hand, 7)

	for i := range hands {
		hands[i] = make([]Hand, 0)
	}

	for _, line := range lines {
		handsLine := strings.Fields(line)

		currHand := handsLine[0]
		currBid, _ := strconv.Atoi(handsLine[1])

		handType := getCardType(currHand)

		hand := Hand{
			hand:     currHand,
			bid:      currBid,
		}

		hands[handType] = append(hands[handType], hand)
	}

	rank := 1

	for _, hand := range hands {
		if len(hand) > 1 {
			sort.Slice(hand, func(i, j int) bool {
				return compareStrings(hand[i].hand, hand[j].hand, scores_1)
			})
		}

		for _, h := range hand {
			res += (h.bid * rank)
			rank += 1
		}
	}

	return res
}

func solve_2(lines []string) int {
	res := 0

	// index = iota of card type
	hands := make([][]Hand, 7)

	for i := range hands {
		hands[i] = make([]Hand, 0)
	}

	for _, line := range lines {
		handsLine := strings.Fields(line)

		currHand := handsLine[0]
		currBid, _ := strconv.Atoi(handsLine[1])

		var handType int

		if strings.Contains(currHand, "J") {
			for k := range scores_2 {
				newCurrHand := strings.ReplaceAll(currHand, "J", string(k))
				handType = max(handType, getCardType(newCurrHand))
			}
		} else {
			handType = getCardType(currHand)
		}

		hand := Hand{
			hand:     currHand,
			bid:      currBid,
		}

		hands[handType] = append(hands[handType], hand)
	}

	rank := 1

	for _, hand := range hands {
		if len(hand) > 1 {
			sort.Slice(hand, func(i, j int) bool {
				return compareStrings(hand[i].hand, hand[j].hand, scores_2)
			})
		}

		for _, h := range hand {
			res += (h.bid * rank)
			rank += 1
		}
	}

	return res
}

func getCardType(hand string) int {
	labelCounts := make(map[rune]int)

	for _, card := range hand {
		labelCounts[card]++
	}

	// Check for Five of a kind
	for _, count := range labelCounts {
		if count == 5 {
			return FIVE_OF_KIND
		}
	}

	// Check for Four of a kind
	for _, count := range labelCounts {
		if count == 4 {
			return FOUR_OF_KIND
		}
	}

	// Check for Full house
	var hasTwo, hasThree bool
	for _, count := range labelCounts {
		if count == 2 {
			hasTwo = true
		}
		if count == 3 {
			hasThree = true
		}
	}
	if hasTwo && hasThree {
		return FULL_HOUSE
	}

	// Check for Three of a kind
	for _, count := range labelCounts {
		if count == 3 {
			return THREE_OF_KIND
		}
	}

	// Check for Two pair
	var pairCount int
	for _, count := range labelCounts {
		if count == 2 {
			pairCount++
		}
	}
	if pairCount == 2 {
		return TWO_PAIR
	}

	// Check for One pair
	if pairCount == 1 {
		return ONE_PAIR
	}

	// If none of the above conditions are met it's a High card
	return HIGH_CARD
}

// Compares scores of strings char by char
func compareStrings(s1, s2 string, scores map[rune]int) bool {
	for i := 0; i < len(s1); i++ {
		if s1[i] != s2[i] {
			return scores_1[rune(s2[i])] > scores_1[rune(s1[i])]
		}

	}

	return true
}
