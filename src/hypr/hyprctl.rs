use super::{Client, Hyprctl, Workspace};
use anyhow::{Context, Result};
use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

impl Hyprctl {
    pub fn connect() -> Result<Hyprctl> {
        let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .context("Failed to find $HYPRLAND_INSTANCE_SIGNATURE")?;
        let root =
            env::var("XDG_RUNTIME_DIR").context("Failed to find $HYPRLAND_INSTANCE_SIGNATURE")?;
        let directory = format!("{}/hypr/{}/.socket.sock", root, his);
        let stream = UnixStream::connect(directory.clone())
            .with_context(|| format!("Can not connect to socket: {}", directory))?;
        Ok(Hyprctl { stream })
    }

    pub fn send_cmd(&mut self, cmd: &str) -> Result<String> {
        self.stream.write_all(cmd.as_bytes())?;
        let mut response = String::new();
        self.stream.read_to_string(&mut response)?;
        Ok(response)
    }

    pub fn get_clients(&mut self) -> Result<Vec<Client>> {
        let res = self
            .send_cmd("j/clients")
            .context("Error sending command: hyprctl -j clients")?;
        let clients: Vec<Client> =
            serde_json::from_str(&res).context(format!("Parsing client data: \n\t{}", res))?;

        Ok(clients)
    }

    pub fn get_workspaces(&mut self) -> Result<Vec<Workspace>> {
        let res = self
            .send_cmd("j/workspaces")
            .context("Error sending command: hyprctl -j workspaces")?;
        let workspaces: Vec<Workspace> =
            serde_json::from_str(&res).context(format!("Parsing workspace data: \n\t{}", res))?;

        Ok(workspaces)
    }
}
