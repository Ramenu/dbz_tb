package main

import (
	"net/http"
	"strings"
	"strconv"
	"fmt"
)

const dokkanWikiLink = "https://dbz-dokkanbattle.fandom.com"

type Unit struct {
	Url string `json:"URL,omitempty"`
	Icon string `json:"Icon,omitempty"`
	Art string `json:"Art,omitempty"`
	Name string `json:"Name,omitempty"`
	Rarity string `json:"Rarity,omitempty"`
	Typ string `json:"Type,omitempty"`
	LeaderSkill string `json:"Leader Skill,omitempty"`
	PassiveSkill string `json:"Passive Skill,omitempty"`
	ActiveSkill string `json:"Active Skill,omitempty"`
	SuperAtk string `json:"Super attack,omitempty"`
	UltraSuperAtk string `json:"Ultra Super Attack,omitempty"`
	UnitSuperAtk string `json:"Unit Super Attack,omitempty"`
	UnitSuperAtkCondition string `json:"Unit Super Attack Condition,omitempty"`
	TransformationCondition string `json:"Transformation Condition,omitempty"`
	AwakensInto string `json:"Awakens Into,omitempty"`
	Categories []string `json:"Categories,omitempty"`
	Atk uint `json:"ATK,omitempty"`
	Def uint `json:"DEF,omitempty"`
	Hp uint `json:"HP,omitempty"`
}

func isAprilFoolsYamchaCard(name string) bool {
	return name == "Flash of Glory Yamcha" ||
	       name == "Absolute Loss Yamcha" ||
		   name == "Gallant Fighter Yamcha" ||
		   name == "Fierce Declaration of War Yamcha" ||
		   name == "Go-Ahead Home Run Yamcha"
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
			fullUnitURL := dokkanWikiLink + unit[1]
			infoResponse, infoErr := http.Get(fullUnitURL)

			if infoErr == nil {
				infoResponseBody := getPageTemplate(&infoResponse.Body)
				if isValidResponse(infoResponseBody) {

					var unitCategories, unitType, unitUltraSa, unitUnitSA, unitUnitSAActivation, unitActiveSkill, unitTransCondition, unitIcon, unitFullImg, awakensInto string
					unitName := removeHTMLTags(GetNameReg().FindStringSubmatch(infoResponseBody)[1])
					unitRarity := removeHTMLTags(GetRarityReg().FindStringSubmatch(infoResponseBody)[2])
					unitLeaderSkill := removeHTMLTags(replaceHTMLTypeIcons(GetLeaderSkillReg().FindStringSubmatch(infoResponseBody)[1]))
					unitSa := removeHTMLTags(GetSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
					unitPassiveSkill := removeHTMLTags(replaceHTMLTypeIcons(GetPassiveReg().FindStringSubmatch(infoResponseBody)[1]))
					unitHP, _ := strconv.Atoi(removeHTMLTags(GetHPReg().FindStringSubmatch(infoResponseBody)[1]))
					unitATK, _ := strconv.Atoi(removeHTMLTags(GetATKReg().FindStringSubmatch(infoResponseBody)[1]))
					unitDEF, _ := strconv.Atoi(removeHTMLTags(GetDEFReg().FindStringSubmatch(infoResponseBody)[1]))
					
					if GetDokkanAwakenIntoReg().MatchString(infoResponseBody) {
						awakensInto = dokkanWikiLink + removeHTMLTags(GetDokkanAwakenIntoReg().FindStringSubmatch(infoResponseBody)[1])
					}

					// Some pages may not have a category for a profile or a type
					if GetCategoryReg().MatchString(infoResponseBody) && GetTypeIconNoOptReg().MatchString(infoResponseBody) {
						unitCategories = removeHTMLTags(GetCategoryReg().FindStringSubmatch(infoResponseBody)[1])
						unitType = removeHTMLTags(replaceHTMLTypeIcons(GetTypeIconNoOptReg().FindStringSubmatch(infoResponseBody)[2]))
					}

					if unitRarity == "LR" {
						unitUltraSa = removeHTMLTags(GetUltraSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
						unitIcon = removeHTMLTags(GetUnitIconReg().FindStringSubmatch(infoResponseBody)[2])
						unitFullImg = removeHTMLTags(GetUnitFullImgReg().FindStringSubmatch(infoResponseBody)[3])
					} else {
						// The 'joke' yamcha cards have a animated image different from non-LR cards, so they have to be extracted
						// differently.
						if !isAprilFoolsYamchaCard(unitName) {
							unitFullImg = removeHTMLTags(GetUnitFullImgReg().FindStringSubmatch(infoResponseBody)[1])
						} else {
							unitFullImg = removeHTMLTags(GetUnitFullImgReg().FindStringSubmatch(infoResponseBody)[2])
						}
						unitIcon = removeHTMLTags(GetUnitIconReg().FindStringSubmatch(infoResponseBody)[1])
					}

					if strings.Contains(infoResponseBody, "Unit_SA") {
						unitUnitSA = removeHTMLTags(GetUnitSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
						unitUnitSAActivation = removeHTMLTags(GetUnitSAActivationReg().FindStringSubmatch(infoResponseBody)[1])
					}

					if strings.Contains(infoResponseBody, "Active_skill.png") {
						unitActiveSkill = removeHTMLTags(GetActiveSkillReg().FindStringSubmatch(infoResponseBody)[1])
					}

					if GetTransformationConditionReg().MatchString(infoResponseBody) {
						unitTransCondition = removeHTMLTags(GetTransformationConditionReg().FindStringSubmatch(infoResponseBody)[1])
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
								   Url: fullUnitURL,
								   Art: unitFullImg,
								   Icon: unitIcon,
								   Name: unitName,
								   Rarity: unitRarity,
								   Typ: unitType,
								   LeaderSkill: unitLeaderSkill,
								   PassiveSkill: unitPassiveSkill,
								   ActiveSkill: unitActiveSkill,
								   SuperAtk: unitSa,
								   UltraSuperAtk: unitUltraSa,
								   UnitSuperAtk: unitUnitSA,
								   UnitSuperAtkCondition: unitUnitSAActivation,
								   TransformationCondition: unitTransCondition,
								   Categories: strings.Split(unitCategories, " - "),
								   AwakensInto: awakensInto,
								   Atk: uint(unitATK),
								   Def: uint(unitDEF),
								   Hp: uint(unitHP)})
					
				}
			}

		}
	}
	return units
}

func (unit Unit) String() string {
	return fmt.Sprint("\nName: ", unit.Name,
					   "\nRarity: ", unit.Rarity,
					   "\nType: ", unit.Typ,
					   "\nLeader skill: ", unit.LeaderSkill,
					   "\nSuper attack: ", unit.SuperAtk,
					   "\nUltra super attack: ", unit.UltraSuperAtk,
					   "\nUnit super attack: ", unit.UnitSuperAtk,
					   "\nUnit super attack activation conditions: ", unit.UnitSuperAtkCondition,
					   "\nActive skill: ", unit.ActiveSkill,
					   "\nPassive skill: ", unit.PassiveSkill,
					   "\nCategories: ", unit.Categories,
					   "\nHP: ", unit.Hp,
					   "\nATK: ", unit.Atk,
					   "\nDEF: ", unit.Def)
}