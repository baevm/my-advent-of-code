package helpers

import (
	"bufio"
	"os"
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
