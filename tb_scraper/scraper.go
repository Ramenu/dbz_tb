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
	url string
	response *http.Response
	err error
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

	var allUnitNames []string
	var allRarityNames []string
	nameReg := GetNameReg()
	rarityReg := GetRarityReg()
	saReg := GetSuperAtkReg()
	//var allSuperAttackNames []string
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

		for isValidPage(responseBody){
			
			urlNameMatches := nameReg.FindAllStringSubmatch(responseBody, -1)
			rarityMatches := rarityReg.FindAllStringSubmatch(responseBody, -1)
			for i := range urlNameMatches {
				// To ensure only names are appended into the slice
				if !strings.Contains(urlNameMatches[i][2], "img alt="){ 
					allUnitNames = append(allUnitNames, fixHTMLSequences(urlNameMatches[i][2]))
				} 

				// Get the other info from the character's wiki link
				if !strings.Contains(urlNameMatches[i][1], "Category"){
					fullInfoURL := "https://dbz-dokkanbattle.fandom.com" + urlNameMatches[i][1]
					infoResponse, infoErr := http.Get(fullInfoURL)
					if infoErr == nil {
						infoResponseBody := getPageTemplate(&infoResponse.Body)
						if isValidPage(infoResponseBody){
							//fmt.Println("Link: ", fullInfoURL)
							unitDescription := saReg.FindAllStringSubmatch(infoResponseBody, -1)
							saMatch := unitDescription[1][1] // This gets the super attack
							fmt.Println("Super attack: ", removeHTMLTags(saMatch))
						}
					}

				}
			}

			for i := range rarityMatches {
				allRarityNames = append(allRarityNames, rarityMatches[i][1])
			}

			page = updateURL(&end, strconv.Itoa(i))
			responseBody = getPageTemplate(&page.response.Body)
		}
	}

	
	/*for _, name := range allUnitNames {
		fmt.Println(name)
	}*/

	/*for _, category := range allRarityNames {
		fmt.Println(category)
	}*/
}