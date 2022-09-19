package main

import "regexp"

func GetNameReg() *regexp.Regexp {
	const nameReg = "<td><a href=\".*\" title=\".*\">(.*)</a>"
	return regexp.MustCompile(nameReg)
}