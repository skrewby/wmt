package workspace

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

type Workspace struct {
	Id           int
	Monitor      int
	Windows      int
	Window       string
	Window_title string
}

func ParseHyprWorkspaceData(str string) []Workspace {
	var workspaces []Workspace

	workspaces_str := strings.Split(str, "\n\n")
	for _, workspace_str := range workspaces_str {
		ws, err := parseWorkspaceStr(workspace_str)
		if err != nil {
			continue
		}
		workspaces = append(workspaces, ws)
	}

	return workspaces
}

func parseWorkspaceStr(str string) (Workspace, error) {
	lines := strings.Split(str, "\n")

	if len(lines) != 6 {
		return Workspace{}, errors.New("Not a valid workspace string")
	}

	id, err := strconv.Atoi(match("(", ")", lines[0]))
	if err != nil {
		fmt.Println("Workspace ID is not an integer")
		id = -1
	}

	monitor := getFieldValueInt(lines[1], "monitorID")
	windows := getFieldValueInt(lines[2], "windows")
	window_hex := getFieldValueStr(lines[4], "lastwindow")
	window, _ := strings.CutPrefix(window_hex, "0x")
	window_title := getFieldValueStr(lines[5], "lastwindowtitle")

	ws := Workspace{
		id,
		monitor,
		windows,
		window,
		window_title,
	}

	return ws, nil
}

func (ws Workspace) Print() {
	fmt.Println("Workspace", ws.Id)
	fmt.Println("\tMonitor:", ws.Monitor)
	fmt.Println("\tNum Windows:", ws.Windows)
	fmt.Println("\tWindow ID:", ws.Window)
	fmt.Println("\tWindow Title:", ws.Window_title)
}

func getFieldValueStr(line string, field string) string {
	sep := fmt.Sprintf("%s: ", field)
	val, found := strings.CutPrefix(strings.TrimSpace(line), sep)
	if !found {
		return ""
	}

	return val

}

func getFieldValueInt(line string, field string) int {
	sep := fmt.Sprintf("%s: ", field)
	after, _ := strings.CutPrefix(strings.TrimSpace(line), sep)
	val, err := strconv.Atoi(after)
	if err != nil {
		fmt.Println("Monitor ID is not an integer")
		val = -1
	}
	return val
}

func match(start, end, s string) string {
	i := strings.Index(s, start)
	if i >= 0 {
		j := strings.Index(s[i:], end)
		if j >= 0 {
			return s[i+len(start) : i+j]
		}
	}
	return ""
}
