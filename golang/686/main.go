package main

import "strings"
import "fmt"

func repeatedStringMatch(A string, B string) int {
    oldlen := len(A)
    AA := A
    for len(AA) < len(B) {
       AA = AA + A
    }
    if strings.Contains(AA,B) {
    	return len(AA) / oldlen
    } else {
    	AA = AA + A
    	if strings.Contains(AA,B) {
    		return len(AA) / oldlen
    	}else {
    		return -1
    	}
    }
    return -1
}

func main() {
	fmt.Println(repeatedStringMatch("abcd","cdabcdab"))

}
