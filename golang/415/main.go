package main 

import "fmt"

func toint(num string) int {
	result := 0
	for _,x := range num {
		result = 10*result +(int(x) - 48)
	}
	return result
}


func reverseString(s string) string {
    runes := []rune(s)
    for from, to := 0, len(runes)-1; from < to; from, to = from+1, to-1 {
        runes[from], runes[to] = runes[to], runes[from]
    }
    return string(runes)
}

func tostring(num int) string {
	table := make(map[int]string,10)
	table[0]= "0"
	table[1]= "1"
	table[2]= "2"
	table[3]= "3"
	table[4]= "4"
	table[5]= "5"
	table[6]= "6"
	table[7]= "7"
	table[8]= "8"
	table[9]= "9"
	return table[num]
}


func addStrings(num1 string, num2 string) string {
	i := int(len(num1) - 1) 
	j := int(len(num2) - 1)
	carry := 0
	res := ""
	for i>=0|| j >=0 || carry > 0 {
		sum := 0
		if (i >= 0) {
			sum = sum + toint(string(num1[i]))
			i = i - 1
		}
		if (j >= 0) {
			sum = sum + toint(string(num2[j]))
			j = j - 1
		}
		sum = sum + carry
		carry = sum / 10
		sum = sum % 10
		res = res + tostring(sum)
	}
	return reverseString(res)
}

func main (){
	fmt.Println(addStrings("3876620623801494171","6529364523802684779"))
}