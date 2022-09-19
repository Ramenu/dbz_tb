package main

import "regexp"

func GetNameReg() *regexp.Regexp {
	const nameReg = "<td><a href=\".*\" title=\".*\">(.*)</a>"
	return regexp.MustCompile(nameReg)
}

func getRarityReg() *regexp.Regexp {
	const categoryReg = "<a href=\"/wiki/Category:[NRSUL]?[NSR]?[NR]\" title=\"Category:([NRSUL]?[NSR]?[NR])\">"
	return regexp.MustCompile(categoryReg)
}