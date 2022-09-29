package main

import (
	"regexp"
	"strings"
)

const urlRegPattern = "<td><a href=\"(.*)\" title=\".*\">.*</a>"
const rarityRegPattern = "<td>.*?(<center>|<span>)<a href=\"/wiki/Category:[NRSUL]?[SR]?R?\" title=\"Category:([NRSUL]?[SR]?R?)\">" // fix this?
const leaderSkillRegPattern = "(?s)Leader_Skill\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const superAtkRegPattern = "(?s)Super_atk\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const ultraSuperAtkRegPattern = "(?s)Ultra_Super_atk\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const unitSuperAtkRegPattern = "(?s)Unit_SA\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const citationRegPattern = "\\[\\d\\]"
const typeIconRegPattern = "<a href=\"\\/wiki\\/Category:(Super|Extreme)?_?[ASPIT][GTHNE][LRYTQ]\" title=\"Category:((Super|Extreme)? ?[ASPIT][GTHNE][LRYTQ])\">"

// This is equivalant to the regular type icon reg pattern, but SUPER or EXTREME is necessary in the type
const typeIconRegPatternNoOpt = "<a href=\"\\/wiki\\/Category:(Super|Extreme)_[ASPIT][GTHNE][LRYTQ]\" title=\"Category:((Super|Extreme) [ASPIT][GTHNE][LRYTQ])\">"

const passiveRegPattern = "(?s)Passive_skill\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const categoryRegPattern = "(?s)Category\\.png.*?<tr>.*?<td colspan=\"2\">(.*?)</td>.*?</tr>"
const nameRegPattern = "(?s)<center>.*?<b>(.*?)</center>.*?</td>.*?</tr>"

// This pattern is meant to be appended to other patterns (just because the wiki can have a lot of these on the same page so more distinction is necessary)
const activationRegPattern = "Activation_Condition\\.png.*?<tr><td colspan=\"2\"><center>(.*?)</center>" 
const unitSAActivationRegPattern = "(?s)Super Attack Multipliers.*?" + activationRegPattern

const activeSkillRegPattern = "(?s)Active_skill\\.png.*?<tr><td colspan=\"2\"><center>(.*?)</center>"

// Some pages have the active skill condition inside the active skill itself, while some dont (annoying but whatever) this regex is for the condition being
// outside the active skill
const activeSkillRegPatternCondition = "(?s)Active Skills.*?" + activationRegPattern

const statRegPattern = "<td><center>(.*?)</center></td>"
const hpRegPattern = "(?s)HP\\.png.*?" + statRegPattern
const atkRegPattern = "(?s)Atk\\.png.*?" + statRegPattern
const defRegPattern = "(?s)Def\\.png.*?" + statRegPattern

const transformationConditionRegPattern = "(?s)Transformation_Condition\\.png.*?<tr><td colspan=\"2\"><center>(.*?)</center>"

var urlReg = regexp.MustCompile(urlRegPattern)
var nameReg = regexp.MustCompile(nameRegPattern)
var rarityReg = regexp.MustCompile(rarityRegPattern)
var leaderSkillReg = regexp.MustCompile(leaderSkillRegPattern)
var superAtkReg = regexp.MustCompile(superAtkRegPattern)
var ultraSuperAtkReg = regexp.MustCompile(ultraSuperAtkRegPattern)
var unitSuperAtkReg = regexp.MustCompile(unitSuperAtkRegPattern)
var citationReg = regexp.MustCompile(citationRegPattern)
var typeIconReg = regexp.MustCompile(typeIconRegPattern)
var passiveReg = regexp.MustCompile(passiveRegPattern)
var categoryReg = regexp.MustCompile(categoryRegPattern)
var typeIconNoOptReg = regexp.MustCompile(typeIconRegPatternNoOpt)
var unitSAActivationReg = regexp.MustCompile(unitSAActivationRegPattern)
var activeSkillReg = regexp.MustCompile(activeSkillRegPattern)
var activeSkillConditionReg = regexp.MustCompile(activeSkillRegPatternCondition)
var hpReg = regexp.MustCompile(hpRegPattern)
var atkReg = regexp.MustCompile(atkRegPattern)
var defReg = regexp.MustCompile(defRegPattern)
var transformationConditionReg = regexp.MustCompile(transformationConditionRegPattern)

func GetURLReg() *regexp.Regexp {
	return urlReg
}

func GetNameReg() *regexp.Regexp {
	return nameReg
}

func GetRarityReg() *regexp.Regexp {
	return rarityReg
}

func GetTypeIconReg() *regexp.Regexp {
	return typeIconReg
}

func GetTypeIconNoOptReg() *regexp.Regexp {
	return typeIconNoOptReg
}

func GetLeaderSkillReg() *regexp.Regexp {
	return leaderSkillReg
}

func GetSuperAtkReg() *regexp.Regexp {
	return superAtkReg
}

func GetUltraSuperAtkReg() *regexp.Regexp {
	return ultraSuperAtkReg
}

func GetUnitSuperAtkReg() *regexp.Regexp {
	return unitSuperAtkReg
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

func GetUnitSAActivationReg() *regexp.Regexp {
	return unitSAActivationReg
}

func GetActiveSkillReg() *regexp.Regexp {
	return activeSkillReg
}

func GetUnitActiveSkillConditionReg() *regexp.Regexp {
	return activeSkillConditionReg
}

func GetHPReg() *regexp.Regexp {
	return hpReg
}

func GetATKReg() *regexp.Regexp {
	return atkReg
}

func GetDEFReg() *regexp.Regexp {
	return defReg
}

func GetTransformationConditionReg() *regexp.Regexp {
	return transformationConditionReg
}

// Replaces the HTML type icons with the just the
// name of the type itself.
func replaceHTMLTypeIcons(s string) string {
	if typeIconReg.MatchString(s) {
		types := typeIconReg.FindAllStringSubmatch(s, -1)
		for _, typ := range types {
			found := typeIconReg.FindString(s)
			if found != "" {
				s = strings.Replace(s, found, typ[2], 1)
			}
		}
	}
	return s
}