package hypr

import (
	"errors"
	"fmt"
	"net"
	"os"

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
	// For some reason XDG_RUNTIME_DIR is not working and have to use /tmp/ instead
	addr := fmt.Sprintf("/tmp/hypr/%s/.socket.sock", h.his)

	connection, err := net.Dial("unix", addr)
	if err != nil {
		fmt.Println("Error connecting to socket: ", err)
		return workspaces
	}

	_, err = connection.Write([]byte("workspaces"))
	if err != nil {
		fmt.Println("Error writing to socket: ", err)
		return workspaces
	}

	buffer := make([]byte, 4096)
	msg_size, err := connection.Read(buffer)
	if err != nil {
		fmt.Println("Error while reading socket: ", err)
		return workspaces
	}
	connection.Close()

	data := string(buffer[:msg_size])
	workspaces = ParseHyprWorkspaceData(data)
	return workspaces
}

func (h Hypr) GetHIS() string {
	return h.his
}