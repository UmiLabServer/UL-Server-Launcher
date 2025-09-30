use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub dir_name: String,
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

    pub fn as_str_animated(&self, tick_count: usize) -> String {
        match self {
            ServerStatus::Running => "Running".to_string(),
            ServerStatus::Stopped => "Stopped".to_string(),
            ServerStatus::Starting => {
                let dots = match tick_count % 4 {
                    0 => "",
                    1 => ".",
                    2 => "..",
                    3 => "...",
                    _ => "",
                };
                format!("Starting{}", dots)
            },
            ServerStatus::Error => "Error".to_string(),
        }
    }
}

pub struct App {
    pub servers: Vec<ServerConfig>,
    pub selected_server: usize,
    pub current_tab: usize,
    pub tabs: Vec<&'static str>,
    pub tick_count: usize,
}

impl App {
    pub fn new() -> Self {
        let servers = Self::load_config().unwrap_or_else(|_| vec![]);
        Self {
            servers,
            selected_server: 0,
            current_tab: 0,
            tabs: vec!["Servers", "Preference"],
            tick_count: 0,
        }
    }

    const CONFIG_FILE: &'static str = "servers.json";

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.servers)?;
        fs::write(Self::CONFIG_FILE, json)?;
        Ok(())
    }

    pub fn load_config() -> Result<Vec<ServerConfig>, Box<dyn std::error::Error>> {
        if !Path::new(Self::CONFIG_FILE).exists() {
            return Err("Config file does not exist".into());
        }
        let content = fs::read_to_string(Self::CONFIG_FILE)?;
        let servers: Vec<ServerConfig> = serde_json::from_str(&content)?;
        Ok(servers)
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
            let _ = self.save_config();
        }
    }

    pub fn tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);
    }

    pub fn add_server(&mut self, config: ServerConfig) {
        self.servers.push(config);
        let _ = self.save_config();
    }

    pub fn remove_server(&mut self, index: usize) {
        if index < self.servers.len() {
            self.servers.remove(index);
            if self.selected_server >= self.servers.len() && !self.servers.is_empty() {
                self.selected_server = self.servers.len() - 1;
            }
            let _ = self.save_config();
        }
    }

    pub fn update_server(&mut self, index: usize, config: ServerConfig) {
        if index < self.servers.len() {
            self.servers[index] = config;
            let _ = self.save_config();
        }
    }
}