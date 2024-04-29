package main

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/skrewby/wmt/hypr"
	"github.com/skrewby/wmt/tui"
)

func main() {
	client, err := hypr.Connect()
	if err != nil {
		fmt.Printf("Could not connect to the hyprland server: %s", err)
	}

	workspaces := client.Workspaces()
	p := tea.NewProgram(tui.CreateModel(workspaces))
	if _, err := p.Run(); err != nil {
		fmt.Println("Error running the UI: ", err)
		os.Exit(1)
	}
}
