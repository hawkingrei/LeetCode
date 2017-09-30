package main

import "fmt"

type MapSum struct {
    Next map[string]*MapSum
    Value int
    IsWord bool
}


/** Initialize your data structure here. */
func Constructor() MapSum {
    return MapSum{
    	Next: make(map[string]*MapSum),
    	Value: 0,
    	IsWord: false,
    }
}


func (this *MapSum) Insert(key string, val int)  {
    root := this
    for _,char := range key {
    	cha := string(char)
    	next, ok := root.Next[cha]
    	if ok {
    		root = next
    	} else {
    		new := Constructor()
    		root.Next[cha] = &new
    		root = &new
    	}
    }
    root.IsWord = true
    root.Value = val
}

func dfs(root MapSum) int {
	sum :=0
	for _,b := range root.Next {
		sum += dfs(*b)
	}
	return sum + root.Value
}

func (this *MapSum) Sum(prefix string) int {
	root := this
	for _,char := range prefix {
		cha := string(char)
		next,ok := root.Next[cha]
		if ok {
			root = next
		} else {
			return 0
		}
	}
	return dfs(*root)  
}

func main(){
	obj := Constructor()
	obj.Insert("apple", 3)
	param := obj.Sum("ap");
	fmt.Println(param)
	obj.Insert("app", 2)
	param = obj.Sum("ap");
	fmt.Println(param)
}