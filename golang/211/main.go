package main

import "fmt"

type WordTrieNode struct {
	IsWord bool
	Index  int
	Nexts  map[string]WordTrieNode
}

type WordDictionary struct {
	root WordTrieNode
}

/** Initialize your data structure here. */
func Constructor() WordDictionary {
	result := WordDictionary{}
	result.root.Nexts = make(map[string]WordTrieNode)
	return result
}

/** Adds a word into the data structure. */
func (this *WordDictionary) AddWord(word string) {
	startNode := &this.root
	i := 0
	for ; i < len(word); i++ {
		if val, ok := startNode.Nexts[string(word[i])]; ok {
			startNode = &val
		} else {
			break
		}
	}

	for i < len(word) {
		newNode := WordTrieNode{}
		newNode.Nexts = make(map[string]WordTrieNode)
		newNode.Index = i
		if i == len(word)-1 {
			newNode.IsWord = true
		}
		startNode.Nexts[string(word[i])] = newNode
		startNode = &newNode
		i = i + 1
	}
}

func dfs(word string, index int, startNode WordTrieNode) bool {
	if index == len(word)-1 {
		if string(word[index]) == "." {
			for _, value := range startNode.Nexts {
				if value.IsWord {
					return true
				}
			}
			return false
		} else {
			endNode := startNode.Nexts[string(word[index])]
			return endNode.IsWord
		}
	}

	if index >= len(word) {

		return false
	}
	if word[index] == '.' {
		var res bool
		for _, value := range startNode.Nexts {
			res = res || dfs(word, index+1, value)
		}
		return res
	} else {
		if val, ok := startNode.Nexts[string(word[index])]; ok {
			return dfs(word, index+1, val)
		} else {
			return false
		}
	}
}

/** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
func (this *WordDictionary) Search(word string) bool {
	if len(word) == 0 {
		return false
	}
	startNode := this.root
	return dfs(word, 0, startNode)
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AddWord(word);
 * param_2 := obj.Search(word);
 */

func main() {
	obj := Constructor()
	obj.AddWord("abb")
	fmt.Println(obj.Search("a.b"))
}
