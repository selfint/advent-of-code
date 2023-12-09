package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strings"
	"unicode"
)

func getInput() ([]string, error) {
	scanner := bufio.NewScanner(os.Stdin)
	lines := []string{}

	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
	}

	// Check for errors during scanning
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "Error reading standard input:", err)
		return lines, err
	}

	return lines, nil
}

func parseLiteralNumberSuffix(str string) (int, error) {
	suffixes := [9]string{
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine",
	}

	for i, suffix := range suffixes {
		if strings.HasSuffix(str, suffix) {
			return i + 1, nil
		}
	}

	return 0, errors.New("str doesn't end with literal number")
}

func main() {
	input, err := getInput()
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error reading standard input:", err)
		return
	}

	sum := 0

	for i, line := range input {
		first, last := 0, 0
		buffer := []rune{}

		for _, char := range line {
			buffer = append(buffer, char)

			if unicode.IsDigit(char) {
				buffer = []rune{}

				number := int(char - '0')
				if first == 0 {
					first = number
				}

				last = number
			} else {
				number, err := parseLiteralNumberSuffix(string(buffer))
				if err == nil {
					// don't reset buffer, next number could be
					// a substring of current buffer

					if first == 0 {
						first = number
					}

					last = number
				}
			}
		}

		if first == 0 || last == 0 {
			fmt.Println("Failed to get number in line:", i)
			os.Exit(1)
		}

		sum += first*10 + last
	}

	fmt.Println(sum)
}
