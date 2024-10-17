package hypr

import (
	"errors"
	"fmt"
	"net"
	"os"
	"sort"

	. "github.com/skrewby/wmt/workspace"
)

// https://wiki.hyprland.org/IPC/
type Hypr struct {
	// Hyprland Instance Signature (HIS)
	his string
	// Runtime Dir
	dir string
}

func Connect() (Hypr, error) {
	his, ok := os.LookupEnv("HYPRLAND_INSTANCE_SIGNATURE")
	if !ok {
		return Hypr{}, errors.New("Couldn't get $HYPRLAND_INSTANCE_SIGNATURE, are you running Hyprland?")
	}
	dir, ok := os.LookupEnv("XDG_RUNTIME_DIR")
	if !ok {
		return Hypr{}, errors.New("Couldn't get $XDG_RUNTIME_DIR")
	}

	h := Hypr{
		his,
		dir,
	}

	return h, nil
}

func (h Hypr) Workspaces() []Workspace {
	var workspaces []Workspace

	data, err := h.writeToSocket("workspaces")
	if err != nil {
		return workspaces
	}

	workspaces = ParseHyprWorkspaceData(data)

	data, err = h.writeToSocket("clients")
	if err != nil {
		return workspaces
	}
	clients := ParseHyprClientData(data)

	for i, ws := range workspaces {
		ws.AddClientData(clients)
		workspaces[i] = ws
	}

	sort.Slice(workspaces, func(i, j int) bool {
		return workspaces[i].Id < workspaces[j].Id
	})
	return workspaces
}

func (h Hypr) SwitchToWorkspace(id int) {
	cmd := fmt.Sprintf("dispatch workspace %d", id)
	_, _ = h.writeToSocket(cmd)
}

func (h Hypr) writeToSocket(cmd string) (string, error) {
	// For some reason XDG_RUNTIME_DIR is not working and have to use /tmp/ instead
	runtime_dir := os.Getenv("XDG_RUNTIME_DIR")
	addr := fmt.Sprintf("%s/hypr/%s/.socket.sock", runtime_dir, h.his)

	connection, err := net.Dial("unix", addr)
	if err != nil {
		return "", errors.New("Error connecting to socket")
	}

	_, err = connection.Write([]byte(cmd))
	if err != nil {
		return "", errors.New("Error writing to socket")
	}

	buffer := make([]byte, 4096)
	msg_size, err := connection.Read(buffer)
	if err != nil {
		return "", errors.New("Error reading socket")
	}
	connection.Close()

	data := string(buffer[:msg_size])

	return data, nil
}

func (h Hypr) GetHIS() string {
	return h.his
}
