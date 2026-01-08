package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, err := os.ReadFile("./input.txt")
	if err != nil {
		fmt.Println("Can't read string....")
	}
	content := string(input)
	strs := strings.Split(content, "\n")
	res := validator(strs)
	fmt.Println("Result:", res)
}

func getSign(line string) int {
	if string(line[0]) == "L" {
		num := line[1:]
		numint, err := strconv.Atoi(num)
		if err != nil {
			panic(err)
		}
		return (-1 * numint)
	} else if string(line[0]) == "R" {
		num := line[1:]
		numint, err := strconv.Atoi(num)
		if err != nil {
			panic(err)
		}
		return (1 * numint)
	}
	return 0
}

func validator(inp []string) int {
	count := 50
	res := 0

	for _, n := range inp {
		prevcount := count
		if n == "" {
			continue
		}

		count = ((count+getSign(n))%100 + 100) % 100
		if prevcount+getSign(n) > 99 || prevcount+getSign(n) < 0 {
			res++
			continue
		}
		if count == 0 {
			res++
		}
	}
	return res
}
