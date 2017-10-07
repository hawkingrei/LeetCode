package main

import "fmt"

func min(a int, b int) int {
	if a > b {
		return b
	}
	return a
}

func max(a int, b int) int {
	if a < b {
		return b
	}
	return a
}

func maximalSquare(matrix [][]byte) int {
    if len(matrix) == 0 {
        return 0
    }
	dp := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		dp[i] = make([]int, len(matrix[0]))
	}
	res := 0
	for n := 0; n < len(matrix); n = n + 1 {
        if matrix[n][0] == byte(49){
			dp[n][0] = 1
            res = 1
		}
	}
	for n := 0; n < len(matrix[0]); n = n + 1 {
        if matrix[0][n] == byte(49) {
			dp[0][n] = 1
            res = 1 
		}
	}
	for i := 1; i < len(matrix); i = i + 1 {
		for j := 1; j < len(matrix[0]); j = j + 1 {
			if matrix[i][j] == byte(49) {
				dp[i][j] = min(dp[i-1][j-1], min(dp[i-1][j], dp[i][j-1])) + 1
			}
			res = max(res, dp[i][j])

		}
	}
	return res * res
}

func main() {
	fmt.Println(maximalSquare([][]byte{{49, 48, 49, 48, 48}, {49, 48, 49, 49, 49}, {49, 49, 49, 49, 49}, {49, 48, 48, 49, 48}}))
}
