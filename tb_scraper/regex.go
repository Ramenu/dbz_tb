package main

import (
	"regexp"
	"strings"
)

const urlRegPattern = "<td><a href=\"(.*)\" title=\".*\">.*</a>"
const rarityRegPattern = "<a href=\"/wiki/Category:[NRSUL]?[NSR]?[NR]\" title=\"Category:([NRSUL]?[NSR]?[NR])\">" // fix this?
const leaderSkillRegPattern = "(?s)Leader_Skill\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const superAtkRegPattern = "(?s)Super_atk\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const citationRegPattern = "\\[\\d\\]"
const typeIconRegPattern = "<a href=\"\\/wiki\\/Category:\\w*_?[A-Z][A-Z][A-Z]\" title=\"Category:(\\w*? ?[A-Z][A-Z][A-Z])\">"
const passiveRegPattern = "(?s)Passive_skill\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const categoryRegPattern = "(?s)Category\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const nameRegPattern = "(?s)<center>.*?<b>(.*?)</center>.*?</td>.*?</tr>"

var urlReg = regexp.MustCompile(urlRegPattern)
var nameReg = regexp.MustCompile(nameRegPattern)
var rarityReg = regexp.MustCompile(rarityRegPattern)
var leaderSkillReg = regexp.MustCompile(leaderSkillRegPattern)
var superAtkReg = regexp.MustCompile(superAtkRegPattern)
var citationReg = regexp.MustCompile(citationRegPattern)
var typeIconReg = regexp.MustCompile(typeIconRegPattern)
var passiveReg = regexp.MustCompile(passiveRegPattern)
var categoryReg = regexp.MustCompile(categoryRegPattern)

func GetURLReg() *regexp.Regexp {
	return urlReg
}

func GetNameReg() *regexp.Regexp {
	return nameReg
}

func GetRarityReg() *regexp.Regexp {
	return rarityReg
}

func GetLeaderSkillReg() *regexp.Regexp {
	return leaderSkillReg
}

func GetSuperAtkReg() *regexp.Regexp {
	return superAtkReg
}

func GetCitationReg() *regexp.Regexp {
	return citationReg
}

func GetPassiveReg() *regexp.Regexp {
	return passiveReg
}

func GetCategoryReg() *regexp.Regexp {
	return categoryReg
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