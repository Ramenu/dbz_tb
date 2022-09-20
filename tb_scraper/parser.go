package main

import "strings"

// Some s[i]racters are invalid for HTML, so when
// scraping the data it is important to fix the
// sequences into their proper s[i]racters.
func fixHTMLSequences(s string) string {
	if strings.Contains(s, "&#39;") { // '
		return strings.Replace(s, "&#39;", "'", -1)
	} 
	if strings.Contains(s, "&amp;") { // &
		return strings.Replace(s, "&amp;", "&", -1)
	}
	return s
}


// Returns a string with the HTML
// tags removed (Note: at the moment this 
// only handles the href tags).
func removeHTMLTags(s string) string {
	length := len(s)
	var tagStartCount uint64 = 0
	var tagsRemovedS string
	tagEnded := true
	for i := range s {
		if s[i] == '<' {
			tagStartCount++
			// Check if its a closing tag
			if i + 1 < length {
				if s[i + 1] == '/' {
					tagStartCount -= 2 // We just incremented it, so sub by 2 is necessary
					tagEnded = false
				}
			}
		}

		if !tagEnded && s[i] == '>' {
			tagEnded = true
			continue
		}

		if tagStartCount == 0 && tagEnded {
			tagsRemovedS += string(s[i])
		}

	}
	return fixHTMLSequences(tagsRemovedS)
}