package main

import (
	"fmt"

	"github.com/skrewby/wmt/hypr"
)

func main() {
	client, err := hypr.Connect()
	if err != nil {
		fmt.Printf("Could not connect to the hyprland server: %s", err)
	}

	workspaces := client.Workspaces()
	for _, ws := range workspaces {
		ws.Print()
	}
}
