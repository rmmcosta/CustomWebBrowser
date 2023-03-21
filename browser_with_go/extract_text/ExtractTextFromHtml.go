package extract_text

import (
	"strings"
)

func ExtractTextFromHtml(html string) string {
	var sb strings.Builder
	var isInsideTags = true
	for _, r := range html {
		if r == '<' {
			isInsideTags = true
		} else if r == '>' {
			isInsideTags = false
		} else if !isInsideTags {
			sb.WriteRune(r)
		}
	}
	return sb.String()
}
