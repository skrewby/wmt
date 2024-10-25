use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub struct Hypr {
    pub workspaces: Vec<Workspace>,
    pub clients: Vec<Client>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    #[serde(rename = "monitorID")]
    pub monitor_id: u32,
    pub windows: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientWorkspace {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub class: String,
    pub title: String,
    pub workspace: ClientWorkspace,

    #[serde(rename = "focusHistoryID")]
    pub focus_id: u32,
}

impl Hypr {
    pub fn new() -> Result<Hypr> {
        let workspaces = get_workspaces()?;
        let clients = get_clients()?;
        Ok(Hypr {
            workspaces,
            clients,
        })
    }
}

fn connect() -> Result<UnixStream> {
    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .context("Failed to find $HYPRLAND_INSTANCE_SIGNATURE")?;
    let root =
        env::var("XDG_RUNTIME_DIR").context("Failed to find $HYPRLAND_INSTANCE_SIGNATURE")?;
    let directory = format!("{}/hypr/{}/.socket.sock", root, his);
    let stream = UnixStream::connect(directory.clone())
        .with_context(|| format!("Can not connect to socket: {}", directory))?;
    Ok(stream)
}

fn send_cmd(cmd: &str) -> Result<String> {
    let mut stream = connect()?;
    stream.write_all(cmd.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

fn get_clients() -> Result<Vec<Client>> {
    let res = send_cmd("j/clients").context("Error sending command: hyprctl -j clients")?;
    let clients: Vec<Client> =
        serde_json::from_str(&res).context(format!("Parsing client data: \n\t{}", res))?;

    Ok(clients)
}

fn get_workspaces() -> Result<Vec<Workspace>> {
    let res = send_cmd("j/workspaces").context("Error sending command: hyprctl -j workspaces")?;
    let workspaces: Vec<Workspace> =
        serde_json::from_str(&res).context(format!("Parsing workspace data: \n\t{}", res))?;

    Ok(workspaces)
}

pub fn switch_to_workspace(id: u32) -> Result<()> {
    let cmd = format!("dispatch workspace {}", id);
    send_cmd(&cmd).context(format!("Error sending command hyprctl {}", cmd))?;

    Ok(())
}
