package main

import (
	"net/http"
	"strings"
	"strconv"
	"fmt"
)

type Unit struct {
	name string
	rarity string
	typ string
	leaderSkill string
	passiveSkill string
	activeSkill string
	superAtk string
	ultraSuperAtk string
	unitSuperAtk string
	unitSuperAtkCondition string
	categories []string
	atk uint
	def uint
	hp uint
}

// Returns an array of all the units found on the
// given page.
func GetAllInfoOnUnits(page *page) []Unit {

	urlNameMatches := GetURLReg().FindAllStringSubmatch(page.responseBody, -1)
	var units []Unit
	// Iterate through all of the unit names
	for _, unit := range urlNameMatches {
		if !strings.Contains(unit[1], "Category") && !strings.Contains(unit[1], "Wind-up Nutcracker") {

			// Get info from the character's wiki link
			fullUnitURL := "https://dbz-dokkanbattle.fandom.com" + unit[1]
			infoResponse, infoErr := http.Get(fullUnitURL)

			if infoErr == nil {
				infoResponseBody := getPageTemplate(&infoResponse.Body)
				if isValidResponse(infoResponseBody) {

					var unitCategories, unitType, unitUltraSa, unitUnitSA, unitUnitSAActivation, unitActiveSkill string
					unitName := removeHTMLTags(GetNameReg().FindStringSubmatch(infoResponseBody)[1])
					unitRarity := removeHTMLTags(GetRarityReg().FindStringSubmatch(infoResponseBody)[2])
					unitLeaderSkill := removeHTMLTags(replaceHTMLTypeIcons(GetLeaderSkillReg().FindStringSubmatch(infoResponseBody)[1]))
					unitSa := removeHTMLTags(GetSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
					unitPassiveSkill := removeHTMLTags(replaceHTMLTypeIcons(GetPassiveReg().FindStringSubmatch(infoResponseBody)[1]))
					unitHP, _ := strconv.Atoi(removeHTMLTags(GetHPReg().FindStringSubmatch(infoResponseBody)[1]))
					unitATK, _ := strconv.Atoi(removeHTMLTags(GetATKReg().FindStringSubmatch(infoResponseBody)[1]))
					unitDEF, _ := strconv.Atoi(removeHTMLTags(GetDEFReg().FindStringSubmatch(infoResponseBody)[1]))

					// Some pages may not have a category for a profile or a type
					if GetCategoryReg().MatchString(infoResponseBody) && GetTypeIconNoOptReg().MatchString(infoResponseBody) {
						unitCategories = removeHTMLTags(GetCategoryReg().FindStringSubmatch(infoResponseBody)[1])
						unitType = removeHTMLTags(replaceHTMLTypeIcons(GetTypeIconNoOptReg().FindStringSubmatch(infoResponseBody)[2]))
					}

					if unitRarity == "LR" {
						unitUltraSa = removeHTMLTags(GetUltraSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
					}

					if strings.Contains(infoResponseBody, "Unit_SA") {
						unitUnitSA = removeHTMLTags(GetUnitSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
						unitUnitSAActivation = removeHTMLTags(GetUnitSAActivationReg().FindStringSubmatch(infoResponseBody)[1])
					}

					if strings.Contains(infoResponseBody, "Active_skill.png") {
						unitActiveSkill = removeHTMLTags(GetActiveSkillReg().FindStringSubmatch(infoResponseBody)[1])
					}

					if GetUnitActiveSkillConditionReg().MatchString(infoResponseBody) { // SIGH (thanks guys)
						unitActiveCondition := removeHTMLTags(GetUnitActiveSkillConditionReg().FindStringSubmatch(infoResponseBody)[1])
						if unitActiveSkill == "" {
							unitActiveSkill = unitActiveCondition
						} else {
							unitActiveSkill += "; " + unitActiveCondition
						}
					}

					units = append(units, Unit{
								   name: unitName,
								   rarity: unitRarity,
								   typ: unitType,
								   leaderSkill: unitLeaderSkill,
								   passiveSkill: unitPassiveSkill,
								   activeSkill: unitActiveSkill,
								   superAtk: unitSa,
								   ultraSuperAtk: unitUltraSa,
								   unitSuperAtk: unitUnitSA,
								   unitSuperAtkCondition: unitUnitSAActivation,
								   categories: strings.Split(unitCategories, ", "),
								   atk: uint(unitATK),
								   def: uint(unitDEF),
								   hp: uint(unitHP)})
					
				}
			}

		}
	}
	return units
}

func (unit Unit) String() string {
	return fmt.Sprint("\nName: ", unit.name,
					   "\nRarity: ", unit.rarity,
					   "\nType: ", unit.typ,
					   "\nLeader skill: ", unit.leaderSkill,
					   "\nSuper attack: ", unit.superAtk,
					   "\nUltra super attack: ", unit.ultraSuperAtk,
					   "\nUnit super attack: ", unit.unitSuperAtk,
					   "\nUnit super attack activation conditions: ", unit.unitSuperAtkCondition,
					   "\nActive skill: ", unit.activeSkill,
					   "\nPassive skill: ", unit.passiveSkill,
					   "\nCategories: ", unit.categories,
					   "\nHP: ", unit.hp,
					   "\nATK: ", unit.atk,
					   "\nDEF: ", unit.def)
}