package main

import (
	"bufio"
	"fmt"
	"os"
)

func getInput() ([]string, error) {
	scanner := bufio.NewScanner(os.Stdin)
	var lines []string

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

	for _, line := range input {
		fmt.Println(line)
	}
}
