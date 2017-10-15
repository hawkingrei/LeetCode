package main 

import "fmt"

func letterCombinations(digits string) []string {

    table := []string{"abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"}
    var result []string
    if len(digits) == 0 {
    	return result
    }
    letterCombinationsDFS(digits, table, 0, "", &result);
    return result
}

func letterCombinationsDFS(digits string,  dict []string, level int ,out string, res *([]string)) {
	if (level == len(digits)) {
		*res = append(*res,out)
	} else {
		strs := dict[int([]rune(string(digits[level]))[0])-int([]rune("2")[0])]
		for _,s := range strs {
			letterCombinationsDFS(digits,dict,level+1,out+string(s),res)
		}
	}
}

func main() {
	fmt.Println(letterCombinations("23"))
}


