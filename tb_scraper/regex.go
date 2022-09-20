package main

import (
	"regexp"
	"strings"
)


const nameRegPattern = "<td><a href=\"(.*)\" title=\".*\">(.*)</a>"
const rarityRegPattern = "<a href=\"/wiki/Category:[NRSUL]?[NSR]?[NR]\" title=\"Category:([NRSUL]?[NSR]?[NR])\">"
const superAtkRegPattern = "(?s)<tr><td colspan=\"2\">(.*?)</td></tr>"
const citationRegPattern = "\\[\\d\\]";
const typeIconRegPattern = "<a href=\"/wiki/Category:[A-Z][A-Z][A-Z]\" title=\"Category:([A-Z][A-Z][A-Z])\">"

var nameReg = regexp.MustCompile(nameRegPattern)
var rarityReg = regexp.MustCompile(rarityRegPattern)
var superAtkReg = regexp.MustCompile(superAtkRegPattern)
var citationReg = regexp.MustCompile(citationRegPattern)
var typeIconReg = regexp.MustCompile(typeIconRegPattern)

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

// Replaces the HTML type icons with the just the
// name of the type itself.
func replaceHTMLTypeIcons(s string) string {
	if typeIconReg.MatchString(s) {
		types := typeIconReg.FindAllStringSubmatch(s, -1)
		for _, typ := range types {
			found := typeIconReg.FindString(s)
			if found != "" {
				s = strings.Replace(s, found, typ[1], 1)
			}
		}
	}
	return s
}