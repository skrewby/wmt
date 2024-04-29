package tui

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
	w "github.com/skrewby/wmt/workspace"
)

type Model struct {
	workspaces []w.Workspace
	cursor     int
}

func CreateModel(workspaces []w.Workspace) Model {
	return Model{
		workspaces: workspaces,
		cursor:     0,
	}
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit

		// Movement
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.workspaces)-1 {
				m.cursor++
			}
		}
	}

	return m, nil
}

func (m Model) View() string {
	s := ""

	// Iterate over the workspaces
	for i, ws := range m.workspaces {
		cursor := " "
		if m.cursor == i {
			cursor = ">"
		}

		s += fmt.Sprintf("%s %d %s\n", cursor, ws.Id, ws.Window_title)
	}

	return s
}
