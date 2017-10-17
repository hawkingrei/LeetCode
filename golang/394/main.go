package main

import (
	"fmt"
	"strconv"
	"strings"
)

func decodeString(s string) string {
	if !strings.Contains(s, "[") {
		return s
	}

	leftPos := strings.Index(s, "[")
	stack := 0
	rightPos := 0
	for i := leftPos + 1; i < len(s); i++ {
		if string(s[i]) == "[" {
			stack++
		} else if string(s[i]) == "]" {
			stack--
		}
		if stack < 0 {
			rightPos = i
			break
		}
	}
	numberPos := 0
	for i := leftPos - 1; i >= 0; i-- {
		if string(s[i]) >= "0" && string(s[i]) <= "9" {
			numberPos = i
		} else {
			break
		}
	}
	number, _ := strconv.Atoi(s[numberPos:leftPos])

	res := ""
	if numberPos != 0 {
		res = s[:numberPos]
	}
	for i := 0; i < number; i++ {
		res = res + decodeString(s[leftPos+1:rightPos])
	}
	if rightPos < len(s)-1 {
		res = res + decodeString(s[rightPos+1:])
	}
	return res
}

func main() {
	s := "3[a]2[bc]"
	fmt.Println(decodeString(s))
}
