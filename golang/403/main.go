package main

import "fmt"

func canCross(stones []int) bool {
	result := make(map[int]bool)
	return helper(stones, 0, 0, result)
}

func helper(stones []int, pos int, jump int, result map[int]bool) bool {
	n := len(stones)
	key := pos | jump<<11
	if pos >= n-1 {
		return true
	}
	if value, ok := result[key]; ok {
		return value
	}
	for i := pos + 1; i < n; i = i + 1 {
		dist := stones[i] - stones[pos]
		if dist < jump-1 {
			continue
		}
		if dist > jump+1 {
			result[key] = false
			return false
		}
		if helper(stones, i, dist, result) {
			result[key] = true
			return true
		}
	}
	result[key] = false
	return false

}

func count(input map[int]bool, key int) int {
	count := 0
	for k, _ := range input {
		if k == key {
			count = count + 1
		}
	}
	return count
}

func main() {
	a := canCross([]int{0,1,3,5,6,8,12,17})
	fmt.Println(a)
}
