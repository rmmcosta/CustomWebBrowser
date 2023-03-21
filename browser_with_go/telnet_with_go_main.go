package main

import (
	"fmt"
	"os"
	"workspace/extract_text"
	"workspace/make_request"
)

func main() {
	//fmt.Println(len(os.Args))
	if len(os.Args) != 2 {
		panic("Wrong command-line arguments")
	}
	//fmt.Println(os.Args[0])
	//fmt.Println(os.Args[1])
	html := make_request.MakeRequest(os.Args[1])
	bodyWithoutTags := extract_text.ExtractTextFromHtml(html)
	fmt.Println(bodyWithoutTags)
}
