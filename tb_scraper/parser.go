package main

import (
	"strings"
	"github.com/microcosm-cc/bluemonday"
)


var policy = bluemonday.StrictPolicy()
var rePattern = GetCitationReg()

// Some characters are invalid for HTML, so when
// scraping the data it is important to fix the
// sequences into their proper characters.
func fixHTMLSequences(s string) string {
	if strings.Contains(s, "&#39;") { // '
		s = strings.ReplaceAll(s, "&#39;", "'")
	} 
	if strings.Contains(s, "&amp;") { // &
		s = strings.ReplaceAll(s, "&amp;", "&")
	}
	if strings.Contains(s, "&lt;") {
		s = strings.ReplaceAll(s, "&lt;", "<")
	}
	if strings.Contains(s, "&gt;") {
		s = strings.ReplaceAll(s, "&gt;", ">")
	}
	if strings.Contains(s, "&le;") {
		s = strings.ReplaceAll(s, "&le;", "<=")
	}
	if strings.Contains(s, "&ge;") {
		s = strings.ReplaceAll(s, "&ge;", ">=")
	}
	if strings.Contains(s, "&#34;") {
		s = strings.ReplaceAll(s, "&#34;", "\"")
	}
	return s
}



// Returns a string with the HTML tags stripped
// (also fixes HTML character junk).
func removeHTMLTags(s string) string {
	s = policy.Sanitize(s)
	if rePattern.MatchString(s) {
		s = rePattern.ReplaceAllString(s, "")
	}
	return fixHTMLSequences(s)
}

