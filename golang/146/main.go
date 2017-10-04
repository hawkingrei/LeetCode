package main

import (
	"container/list"
	"fmt"
)

type Record struct {
	Key int
	Val int
}

type LRUCache struct {
	size    int
	intlist *list.List
	items   map[int]*list.Element
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		size:    capacity,
		intlist: list.New(),
		items:   make(map[int]*list.Element),
	}
}

func (this *LRUCache) Get(key int) int {
	val, ok := this.items[key]
	if ok {
		this.intlist.MoveToFront(val)
		return val.Value.(*Record).Val
	}
	return -1
}

func (this *LRUCache) Put(key int, value int) {
	val, ok := this.items[key]
	if ok {
		val.Value.(*Record).Val = value
		this.intlist.MoveToFront(val)
		return
	}
	element := this.intlist.PushFront(&Record{Key: key, Val: value})
	this.items[key] = element

	if this.intlist.Len() > this.size {
		this.removeOld()
	}
}

func (this *LRUCache) removeOld() {
	delElem := this.intlist.Back()
	delete(this.items, delElem.Value.(*Record).Key)
	this.intlist.Remove(delElem)
}

func main() {
	cache := Constructor(2)
	cache.Put(1, 1)
	cache.Put(2, 2)
	fmt.Println(cache.Get(1)) // returns 1
	cache.Put(3, 3)           // evicts key 2
	fmt.Println(cache.Get(2)) // returns -1 (not found)
	cache.Put(4, 4)           // evicts key 1
	fmt.Println(cache.Get(1)) // returns -1 (not found)
	fmt.Println(cache.Get(3)) // returns 3
	fmt.Println(cache.Get(4)) // returns 4
}
