package main

import "fmt"

func maxSumOfThreeSubarrays(nums []int, k int) []int {
	sums := make([]int, len(nums)-k+1)
	for _, y := range nums[:k] {
		sums[0] = sums[0] + y
	}
	for x := 1; x <= len(sums)-1; x++ {
		sums[x] = sums[x-1] - nums[x-1] + nums[x+k-1]
	}
	bp := make([]int, 3)
	max := 0
	maxl := make(map[int]int, len(sums))
	maxr := make(map[int]int, len(sums))
	maxltmp := 0
	maxrtmp := 0
	for x, y := range sums {
		if y > maxltmp {
			maxl[x] = x
			maxltmp = sums[x]
		} else {
			maxl[x] = maxl[x-1]
		}
		if sums[len(sums)-1-x] > maxrtmp {
			maxr[len(sums)-1-x] = len(sums) - 1 - x
			maxrtmp = sums[len(sums)-1-x]
		} else {
			maxr[len(sums)-1-x] = maxr[len(sums)-1-x+1]
		}
	}
	fmt.Println(maxr)
	fmt.Println(maxl)
	for x := k; x <= len(sums)-1-k; x++ {
		va := maxr[x+k]
		vb := maxl[x-k]
		if sums[x]+sums[va]+sums[vb] > max {
			max = sums[x] + sums[va] + sums[vb]
			bp[0] = vb
			bp[1] = x
			bp[2] = va
		}
	}
	return bp
}

func main() {
	fmt.Println(maxSumOfThreeSubarrays([]int{1, 2, 1, 2, 6, 7, 5, 1}, 2))
}
