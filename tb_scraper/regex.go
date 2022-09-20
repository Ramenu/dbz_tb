package main

import "regexp"

func GetNameReg() *regexp.Regexp {
	const nameReg = "<td><a href=\"(.*)\" title=\".*\">(.*)</a>"
	return regexp.MustCompile(nameReg)
}

func GetRarityReg() *regexp.Regexp {
	const categoryReg = "<a href=\"/wiki/Category:[NRSUL]?[NSR]?[NR]\" title=\"Category:([NRSUL]?[NSR]?[NR])\">"
	return regexp.MustCompile(categoryReg)
}

func GetSuperAtkReg() *regexp.Regexp {
	const superAtkReg = "(?s)<tr><td colspan=\"2\">(.*?)</td></tr>"
	return regexp.MustCompile(superAtkReg)
}

func replaceHTMLTypeIcons(s string) string {
	return ""
}