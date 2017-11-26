package main

import "fmt"

func longestPalindrome(s string) int {
	fmt.Println(len(s))
	if len(s) == 1 {
		return 1
	}
	querytable := make(map[string]int)
	for _, v := range s {
		if _, ok := querytable[string(v)]; ok {
			querytable[string(v)] = querytable[string(v)] + 1
		} else {
			querytable[string(v)] = 1
		}
	}
	fmt.Println(querytable)
	result := 0
	onlyone := false
	for _, v := range querytable {
		result = result + v
		if v%2 == 1 {
			result = result - 1
			onlyone = true
		}
	}
	if onlyone {
		result = result + 1
	}
	return result
}

func main() {
	fmt.Println(longestPalindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth"))
}
