package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
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

type Game struct {
	maxRed   int
	maxGreen int
	maxBlue  int
}

func parseGame(game string) (Game, error) {
	parts := strings.Split(game, ":")
	if len(parts) != 2 {
		return Game{}, fmt.Errorf("invalid game string: %s", game)
	}

	rawGrabs := parts[1]

	maxRed, maxGreen, maxBlue := 0, 0, 0

	for _, rawGrab := range strings.Split(rawGrabs, ";") {
		for _, rawHandful := range strings.Split(rawGrab, ",") {
			parts := strings.Split(rawHandful, " ")
			if len(parts) != 3 {
				return Game{}, fmt.Errorf("invalid handful '%s': parts=%v", rawHandful, parts)
			}

			rawAmount, rawColor := parts[1], parts[2]
			amount, err := strconv.Atoi(rawAmount)
			if err != nil {
				return Game{}, fmt.Errorf("invalid amount: %s", rawAmount)
			}

			switch rawColor {
			case "red":
				if amount > maxRed {
					maxRed = amount
				}
			case "green":
				if amount > maxGreen {
					maxGreen = amount
				}
			case "blue":
				if amount > maxBlue {
					maxBlue = amount
				}
			default:
				return Game{}, fmt.Errorf("invalid color: %s", rawColor)
			}
		}
	}

	return Game{maxRed, maxGreen, maxBlue}, nil
}

func main() {
	input, err := getInput()
	if err != nil {
		log.Println("Error reading standard input:", err)
		return
	}

	sum := 0

	for i, rawGame := range input {
		game, err := parseGame(rawGame)
		if err != nil {
			log.Printf("Failed to parse line %d '%s': %v\n", i, rawGame, err)
			os.Exit(1)
		}

		power := game.maxRed * game.maxBlue * game.maxGreen

		sum += power
	}

	fmt.Println(sum)
}
