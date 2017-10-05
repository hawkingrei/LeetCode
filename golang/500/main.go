package main

import "fmt"
import "strings"

const (
	FL = iota
	SL
	TL
)

func findWords(words []string) []string {
	firestline := "qwertyuiop"
	secondline := "asdfghjkl"
	thirdline := "zxcvbnm"
	var result []string
	table := make(map[string]int, 26)
	for _, x := range firestline {
		table[string(x)] = FL
	}
	for _, x := range secondline {
		table[string(x)] = SL
	}
	for _, x := range thirdline {
		table[string(x)] = TL
	}
OUT:
	for _, word := range words {
		w := strings.ToLower(string(word))
		flag,_ := table[string(w[0])]
		for _, y := range w {
			if f := table[string(y)]; f != flag {
				continue OUT
			}
		}
		result = append(result, word)
	}
	return result
}

func main() {
	fmt.Println(findWords([]string{"Hello", "Alaska", "Dad", "Peace"}))
}
