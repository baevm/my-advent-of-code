package helpers

import (
	"bufio"
	"log"
	"os"
	"time"
)

func OpenTask(path string) ([]string, error) {
	file, err := os.Open(path)

	if err != nil {
		return nil, err
	}

	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines, scanner.Err()
}

func OpenTaskAsString(path string) (string, error) {
	byteArr, err := os.ReadFile(path)

	if err != nil {
		return "", err
	}

	return string(byteArr), nil
}

func TimeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	log.Printf("%s took %s", name, elapsed)
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}
