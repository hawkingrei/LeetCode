package main

import "fmt"

type NumArray struct {
    Num []int
}


func Constructor(nums []int) NumArray {
    num := make([]int,len(nums))
    total := 0
    for x,y := range nums {
    	total = total + y
    	num[x] = total
    }
    return NumArray{
    	Num: num,
    }
}


func (this *NumArray) SumRange(i int, j int) int {
    if i == 0 {
    	return this.Num[j]
    } else {
    	return this.Num[j] - this.Num[i - 1]
    }
}

func main() {
	obj := Constructor([]int{-2, 0, 3, -5, 2, -1})
    param_1 := obj.SumRange(0,5)
    fmt.Println(param_1)
}