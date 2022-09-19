package main

import "strings"

// Some characters are invalid for HTML, so when
// scraping the data it is important to fix the
// sequences into their proper characters.
func fixHTMLSequences(s string) string {
	if strings.Contains(s, "&#39;") { // '
		return strings.Replace(s, "&#39;", "'", -1)
	} 
	if strings.Contains(s, "&amp;") { // &
		return strings.Replace(s, "&amp;", "&", -1)
	}
	return s
}