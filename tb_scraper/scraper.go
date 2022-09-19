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
func isValidPage(err error, responseBody string) bool {
	emptyPage := strings.Contains(responseBody, "There is currently no text in this page.")
	return !emptyPage
}

// Updates the URL to the 'next' page of characters. 
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

		for isValidPage(err, responseBody){
			
			nameMatches := GetNameReg().FindAllStringSubmatch(responseBody, -1)
			rarityMatches := getRarityReg().FindAllStringSubmatch(responseBody, -1)
			for i := range nameMatches {
				// To ensure only names are appended into the slice
				if !strings.Contains(nameMatches[i][1], "img alt="){ 
					allUnitNames = append(allUnitNames, fixHTMLSequences(nameMatches[i][1]))
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

	for _, category := range allRarityNames {
		fmt.Println(category)
	}
}