package tui

import (
	"strconv"

	"github.com/charmbracelet/bubbles/table"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/skrewby/wmt/hypr"
	w "github.com/skrewby/wmt/workspace"
	"golang.org/x/term"
	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

var baseStyle = lipgloss.NewStyle().
	BorderStyle(lipgloss.NormalBorder()).
	BorderForeground(lipgloss.Color("240"))

type Model struct {
	workspaces []w.Workspace
	client     hypr.Hypr
	cursor     int
	table      table.Model
}

func CreateModel(client hypr.Hypr, workspaces []w.Workspace) Model {
	width, _, _ := term.GetSize(0)
	fill_width := width - 4 - 7 - 7 - 10

	columns := []table.Column{
		{Title: "ID", Width: 4},
		{Title: "Monitor", Width: 7},
		{Title: "Program", Width: 7},
		{Title: "Title", Width: fill_width},
	}
	rows := make([]table.Row, 0)
	for _, ws := range workspaces {
		rows = append(rows, table.Row{strconv.Itoa(ws.Id), strconv.Itoa(ws.Monitor), cases.Title(language.English, cases.NoLower).String(ws.Class), ws.WindowTitle})
	}
	t := table.New(
		table.WithColumns(columns),
		table.WithRows(rows),
		table.WithFocused(true),
		table.WithHeight(len(workspaces)),
	)
	s := table.DefaultStyles()
	s.Header = s.Header.
		BorderStyle(lipgloss.NormalBorder()).
		BorderForeground(lipgloss.Color("240")).
		BorderBottom(true).
		Bold(false)
	s.Selected = s.Selected.
		Foreground(lipgloss.Color("229")).
		Background(lipgloss.Color("57")).
		Bold(false)
	t.SetStyles(s)

	return Model{
		workspaces: workspaces,
		client:     client,
		cursor:     0,
		table:      t,
	}
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd

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

		// Switch workspace
		case "enter", " ":
			if len(m.workspaces) > 0 {
				m.client.SwitchToWorkspace(m.cursor + 1)
			}
			return m, tea.Quit

		case "1", "2", "3", "4", "5", "6", "7", "8", "9":
			r := msg.String()
			sel, err := strconv.Atoi(r)
			if err == nil {
				m.client.SwitchToWorkspace(sel)
			}
			return m, tea.Quit
		}
	}

	m.table, cmd = m.table.Update(msg)
	return m, cmd
}

func (m Model) View() string {
	return baseStyle.Render(m.table.View()) + "\n"
}
