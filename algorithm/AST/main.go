package main

import (
	"errors"
	"fmt"
)

type takonType int

const (
	DIGITAL takonType = iota
	SEPARATOR
	SPACE
	ERROR
)

type takon struct {
	Consult string
	Type    takonType
}

func MakenTakon(c string) (takon, error) {
	switch {
	case c == "0" || c == "1" || c == "2" || c == "3" || c == "4" || c == "5" || c == "6" || c == "7" || c == "8" || c == "9":
		return takon{Consult: c, Type: DIGITAL}, nil
	case c == " ":
		return takon{Consult: c, Type: SPACE}, nil
	case c == "." || c == "e" || c == "E":
		return takon{Consult: c, Type: SEPARATOR}, nil
	default:
		return takon{Consult: c, Type: ERROR}, errors.New("type error")
	}
}

func MakeTakons(c string) ([]takon, error) {
	var result []takon
	for _, y := range c {
		t, err := MakenTakon(string(y))
		if err != nil {
			return result, err
		}
		if len(result) != 0 && t.Type == result[len(result)-1].Type {
			result[len(result)-1].Consult = result[len(result)-1].Consult + t.Consult
		} else {
			result = append(result, t)
		}
	}
	return result, nil
}

func main() {
	t, err := MakeTakons("  1A0")
	if err != nil {
		fmt.Println(err.Error())
	} else {
		fmt.Println(t)
	}

}
