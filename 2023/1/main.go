package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
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

func main() {
	input, err := getInput()
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error reading standard input:", err)
		return
	}

	sum := 0

	for _, line := range input {
		if len(line) == 0 {
			continue
		}

		var first rune
		var last rune

		didFirst := false

		for _, char := range line {
			if unicode.IsDigit(char) {
				if !didFirst {
					first = char
					didFirst = true
				}

				last = char
			}
		}

		number, err := strconv.Atoi(fmt.Sprintf("%c%c", first, last))
		if err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}

		sum += number
	}

	fmt.Println(sum)
}
