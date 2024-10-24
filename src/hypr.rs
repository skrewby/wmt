use std::os::unix::net::UnixStream;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub mod hyprctl;

pub struct Hypr {
    pub workspaces: Vec<Workspace>,
    pub clients: Vec<Client>,
}

pub struct Hyprctl {
    stream: UnixStream,
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
        let workspaces = Hyprctl::connect()?.get_workspaces()?;
        let clients = Hyprctl::connect()?.get_clients()?;
        Ok(Hypr {
            workspaces,
            clients,
        })
    }
}
