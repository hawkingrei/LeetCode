package main 

import "fmt"

func grayCode(n int) []int {
    count := 1 << (uint)(n)
    var code []int
    for i:=0;i<count;i=i+1{
    	code = append(code,binaryToGrayCode(i))
    }
    return code
}

func binaryToGrayCode (n int ) int {
	return n ^ (n >> 1)
}

func main() {
	fmt.Println(grayCode(2))
}