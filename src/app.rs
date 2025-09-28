use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub status: ServerStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServerStatus {
    Running,
    Stopped,
    Starting,
    Error,
}

impl ServerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ServerStatus::Running => "Running",
            ServerStatus::Stopped => "Stopped",
            ServerStatus::Starting => "Starting...",
            ServerStatus::Error => "Error",
        }
    }
}

pub struct App {
    pub servers: Vec<ServerConfig>,
    pub selected_server: usize,
    pub current_tab: usize,
    pub tabs: Vec<&'static str>,
}

impl App {
    pub fn new() -> Self {
        let servers = vec![
            ServerConfig {
                name: "Minecraft Server".to_string(),
                host: "localhost".to_string(),
                port: 25565,
                status: ServerStatus::Stopped,
            },
            ServerConfig {
                name: "Web Server".to_string(),
                host: "0.0.0.0".to_string(),
                port: 8080,
                status: ServerStatus::Running,
            },
            ServerConfig {
                name: "Database Server".to_string(),
                host: "localhost".to_string(),
                port: 5432,
                status: ServerStatus::Starting,
            },
            ServerConfig {
                name: "API Server".to_string(),
                host: "localhost".to_string(),
                port: 3000,
                status: ServerStatus::Error,
            },
        ];

        Self {
            servers,
            selected_server: 0,
            current_tab: 0,
            tabs: vec!["Servers", "Logs", "Settings"],
        }
    }

    pub fn next(&mut self) {
        if !self.servers.is_empty() {
            self.selected_server = (self.selected_server + 1) % self.servers.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.servers.is_empty() {
            if self.selected_server == 0 {
                self.selected_server = self.servers.len() - 1;
            } else {
                self.selected_server -= 1;
            }
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % self.tabs.len();
    }

    pub fn previous_tab(&mut self) {
        if self.current_tab == 0 {
            self.current_tab = self.tabs.len() - 1;
        } else {
            self.current_tab -= 1;
        }
    }

    pub fn select_item(&mut self) {
        if self.current_tab == 0 && !self.servers.is_empty() {
            let server = &mut self.servers[self.selected_server];
            match server.status {
                ServerStatus::Running => server.status = ServerStatus::Stopped,
                ServerStatus::Stopped => server.status = ServerStatus::Starting,
                _ => {}
            }
        }
    }
}