package main

import (
	"fmt"
	"io"
	"net/http"
	"strconv"
	"strings"
)

type page struct {
	url      string
	responseBody string
	err      error
}

func getPageTemplate(responseBody *io.ReadCloser) string {
	bytes, _ := io.ReadAll(*responseBody)
	return string(bytes)
}

// Returns true if the page actually contains anything
// useful.
func isValidPage(page *page) bool {
	emptyPage := strings.Contains(page.responseBody, "There is currently no text in this page.")
	invalidUnit := strings.Contains(page.responseBody, "This character is unreleased, you can only fight it as a boss")

	return !emptyPage && !invalidUnit && page.err == nil
}

// Similar to 'isValidPage' but just checks the body instead.
func isValidResponse(body string) bool {
	emptyPage := strings.Contains(body, "There is currently no text in this page.")
	invalidUnit := strings.Contains(body, "This character is unreleased, you can only fight it as a boss")

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
	responseBody := getPageTemplate(&response.Body)
	return page{newURL, responseBody, err}
}

// 'url' must be a link to a unit page, otherwise the function will
// more than definitely panic. 
func getUnitsFromCharacterList(url string, units chan<-[]Unit) {
	response, err := http.Get(url)
	page := page{url, getPageTemplate(&response.Body), err}
	units <- GetAllInfoOnUnits(&page)
}

// Returns a slice with URLs to all of the unit lists
func getListURLs() []string {
	var urls []string
	fixedUrls := [4]string {
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(1)001_to_(1)100",
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(2)001_to_(2)1000",
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(3)001_to_(3)1000",
		"https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(4)001_to_(4)_unknown"}
	end := 100
	for i, url := range fixedUrls {
		response, err := http.Get(url)
		page := page{responseBody: getPageTemplate(&response.Body), err: err, url: url}
		for isValidPage(&page) {
			urls = append(urls, page.url)
			page = updateURL(&end, strconv.Itoa(i + 1))
		}
	}
	
	return urls
}


func main() {

	urls := getListURLs()
	var unitList [100]chan[]Unit // 100 is excessive, but just for future-proofing i guess
	length := len(urls)

	for i := 0; i < length; i++ {
		unitList[i] = make(chan []Unit)
		go getUnitsFromCharacterList(urls[i], unitList[i])
	}

	for i := 0; i < length; i++ {
		for _, unit := range <-unitList[i] {
			fmt.Println(unit)
		}
	}

}
