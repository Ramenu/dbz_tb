package main

import (
	"regexp"
)


const nameRegPattern = "<td><a href=\"(.*)\" title=\".*\">(.*)</a>"
const rarityRegPattern = "<a href=\"/wiki/Category:[NRSUL]?[NSR]?[NR]\" title=\"Category:([NRSUL]?[NSR]?[NR])\">"
const superAtkRegPattern = "(?s)<tr><td colspan=\"2\">(.*?)</td></tr>"
const citationRegPattern = "\\[\\d\\]";

var nameReg = regexp.MustCompile(nameRegPattern)
var rarityReg = regexp.MustCompile(rarityRegPattern)
var superAtkReg = regexp.MustCompile(superAtkRegPattern)
var citationReg = regexp.MustCompile(citationRegPattern)

func GetNameReg() *regexp.Regexp {
	return nameReg
}

func GetRarityReg() *regexp.Regexp {
	return rarityReg
}

func GetSuperAtkReg() *regexp.Regexp {
	return superAtkReg
}

func GetCitationReg() *regexp.Regexp {
	return citationReg
}


func replaceHTMLTypeIcons(s string) string {
	return ""
}