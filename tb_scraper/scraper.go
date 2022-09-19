package main

import (
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"regexp"
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
	url := "https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(1)001_to_(1)100"
	response, err := http.Get(url)
	page := page{url, response, err}
	end := 100
	responseBody := getPageTemplate(&page.response.Body)
	fmt.Println(responseBody)

	for isValidPage(err, responseBody){
		
		const nameReg = "<td><a href=\".*\" title=\"(.*)\">\\[.*\\].*</a>\\s</td>"
		re := regexp.MustCompile(nameReg)
		nameMatches := re.FindAllStringSubmatch(responseBody, -1)
		for i := range nameMatches {
			fmt.Println(nameMatches[i][1])
		}

		page = updateURL(&end, "1")
		responseBody = getPageTemplate(&page.response.Body)
		fmt.Println("Next URL is: ", page.url)
	}
}