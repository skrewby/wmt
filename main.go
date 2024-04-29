package main

import (
	"fmt"

	"github.com/skrewby/wmt/hypr"
)

func main() {
	_, err := hypr.Connect()
	if err != nil {
		fmt.Printf("Could not connect to the hyprland server: %s", err)
	}
}
