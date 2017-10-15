func judgeCircle(moves string) bool {
	l1:=0
	l2:=0
	for _,move := range moves {
        switch (string(move)) {
		case "R":
		
			l2 = l2 + 1
		case "L":
			l2 = l2 - 1
		case "U":
			l1 = l1 + 1
		case "D":
			l1 = l1 - 1
		}
	}   
	return l1 == 0 && l2 == 0
}