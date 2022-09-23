package main

import (
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"strconv"
	"strings"
)

type page struct {
	url      string
	response *http.Response
	err      error
}

func getPageTemplate(responseBody *io.ReadCloser) string {
	bytes, _ := ioutil.ReadAll(*responseBody)
	return string(bytes)
}

// Returns true if the page actually contains anything
// useful.
func isValidPage(responseBody string) bool {
	emptyPage := strings.Contains(responseBody, "There is currently no text in this page.")
	invalidUnit := strings.Contains(responseBody, "This character is unreleased, you can only fight it as a boss") ||
		strings.Contains(responseBody, "Wind-up Nutcracker")

	return !emptyPage && !invalidUnit
}

// Updates the URL to the 'next' page of units.
// Should be called only after the first URL has
// been gone through.
func updateURL(end *int, seriesNumber string) page {
	newStart := strconv.Itoa(*end + 1)
	*end += 100

	newURL := "https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(" + seriesNumber + ")" + newStart + "_to_(" + seriesNumber + ")" + strconv.Itoa(*end)

	response, err := http.Get(newURL)
	return page{newURL, response, err}
}

func main() {

	urls := [4]string{
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(1)001_to_(1)100",
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(2)001_to_(2)1000",
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(3)001_to_(3)1000",
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(4)001_to_(4)_unknown"}

	for i := range urls {
		response, err := http.Get(urls[i])
		page := page{urls[i], response, err}
		end := 100
		responseBody := getPageTemplate(&page.response.Body)

		for isValidPage(responseBody) {

			urlNameMatches := GetURLReg().FindAllStringSubmatch(responseBody, -1)
			for i := range urlNameMatches {

				if !strings.Contains(urlNameMatches[i][1], "Category") {

					// Get info from the character's wiki link
					fullInfoURL := "https://dbz-dokkanbattle.fandom.com" + urlNameMatches[i][1]
					infoResponse, infoErr := http.Get(fullInfoURL)
					if infoErr == nil {
						infoResponseBody := getPageTemplate(&infoResponse.Body)
						if isValidPage(infoResponseBody) {

							var unitCategories, unitType, unitUltraSa string
							unitName := removeHTMLTags(GetNameReg().FindStringSubmatch(infoResponseBody)[1])
							unitRarity := removeHTMLTags(GetRarityReg().FindStringSubmatch(infoResponseBody)[2])
							unitLeaderSkill := removeHTMLTags(replaceHTMLTypeIcons(GetLeaderSkillReg().FindStringSubmatch(infoResponseBody)[1]))
							unitSa := removeHTMLTags(GetSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
							unitPassiveSkill := removeHTMLTags(replaceHTMLTypeIcons(GetPassiveReg().FindStringSubmatch(infoResponseBody)[1]))

							// Some pages may not have a category for a profile or a type
							if GetCategoryReg().MatchString(infoResponseBody) && GetTypeIconNoOptReg().MatchString(infoResponseBody) { 
								unitCategories = removeHTMLTags(GetCategoryReg().FindStringSubmatch(infoResponseBody)[1])
								unitType = removeHTMLTags(replaceHTMLTypeIcons(GetTypeIconNoOptReg().FindStringSubmatch(infoResponseBody)[2]))
							} 

							if unitRarity == "LR" {
								unitUltraSa = removeHTMLTags(GetUltraSuperAtkReg().FindStringSubmatch(infoResponseBody)[1])
							}
							
							fmt.Println("URL: ", fullInfoURL,
							    "\nName: ", unitName,
								"\nRarity: ", unitRarity,
								"\nType: ", unitType,
								"\nLeader skill: ", unitLeaderSkill,
								"\nSuper attack: ", unitSa,
								"\nUltra super attack: ", unitUltraSa,
								"\nPassive skill: ", unitPassiveSkill,
							    "\nCategories: ", unitCategories, "\n")
						}
					}

				}
			}

			page = updateURL(&end, strconv.Itoa(i))
			responseBody = getPageTemplate(&page.response.Body)
		}
	}
}
