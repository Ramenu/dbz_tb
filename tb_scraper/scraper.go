package main

import (
	"encoding/json"
	"io"
	"net/http"
	"os"
	"strconv"
	"strings"
)

type page struct {
	url      string
	responseBody string
	err      error
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func getPageTemplate(responseBody *io.ReadCloser) string {
	bytes, err := io.ReadAll(*responseBody)
	check(err)
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
	//listLength := 137
	file, err := os.OpenFile("./units.json", os.O_WRONLY|os.O_TRUNC, os.ModeAppend)
	check(err)
	defer file.Close()
	_, e := file.Write([]byte("[\n"))
	check(e)
	for i := 0; i < length; i++ {
		var n int
		list := <-unitList[i]
		// Ensure we only get the length when its necessary
		if i + 1 == length {
			n = len(list)
		}
		for j, unit := range list {
			b, err := json.MarshalIndent(unit, "", " ")
			check(err)
			if i + 1 == length && j + 1 == n {
				_, e = file.Write(b)
			} else {
				_, e = file.Write([]byte(string(b) + ",\n"))
			}
			check(e)
		}
	}
	_, e = file.Write([]byte("\n]"))
	check(e)

}
