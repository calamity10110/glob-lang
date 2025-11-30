// GUL TUI IDE - Terminal User Interface IDE for GUL
// Built with Ratatui (formerly tui-rs)

use std::io;
use std::path::PathBuf;

/// Main TUI IDE application state
pub struct GulTuiIde {
    /// Current file being edited
    current_file: Option<PathBuf>,
    /// Editor buffer content
    buffer: String,
    /// Cursor position (line, column)
    cursor: (usize, usize),
    /// File browser state
    file_browser: FileBrowser,
    /// Command palette state
    command_palette: CommandPalette,
    /// Status bar message
    status_message: String,
    /// Whether the IDE is running
    running: bool,
}

/// File browser component
pub struct FileBrowser {
    /// Current directory
    current_dir: PathBuf,
    /// List of files in current directory
    files: Vec<PathBuf>,
    /// Selected file index
    selected: usize,
    /// Whether the browser is visible
    visible: bool,
}

/// Command palette for quick actions
pub struct CommandPalette {
    /// Search query
    query: String,
    /// Available commands
    commands: Vec<Command>,
    /// Selected command index
    selected: usize,
    /// Whether the palette is visible
    visible: bool,
}

/// IDE command
#[derive(Clone, Debug)]
pub struct Command {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Command action
    pub action: CommandAction,
}

/// Command actions
#[derive(Clone, Debug)]
pub enum CommandAction {
    /// Open file
    OpenFile,
    /// Save file
    SaveFile,
    /// Build project
    Build,
    /// Run project
    Run,
    /// Format code
    Format,
    /// Show Git status
    GitStatus,
    /// Quit IDE
    Quit,
}

impl GulTuiIde {
    /// Create a new TUI IDE instance
    pub fn new() -> Self {
        GulTuiIde {
            current_file: None,
            buffer: String::new(),
            cursor: (0, 0),
            file_browser: FileBrowser::new(),
            command_palette: CommandPalette::new(),
            status_message: "Welcome to GUL TUI IDE".to_string(),
            running: true,
        }
    }

    /// Run the IDE main loop
    pub fn run(&mut self) -> io::Result<()> {
        // Initialize terminal
        self.status_message = "TUI IDE initialized. Press Ctrl+P for commands.".to_string();

        // Main event loop would go here
        // For now, this is a placeholder

        Ok(())
    }

    /// Open a file
    pub fn open_file(&mut self, path: PathBuf) -> io::Result<()> {
        self.buffer = std::fs::read_to_string(&path)?;
        self.current_file = Some(path);
        self.cursor = (0, 0);
        self.status_message = format!("Opened: {:?}", self.current_file);
        Ok(())
    }

    /// Save current file
    pub fn save_file(&mut self) -> io::Result<()> {
        if let Some(ref path) = self.current_file {
            std::fs::write(path, &self.buffer)?;
            self.status_message = format!("Saved: {:?}", path);
        }
        Ok(())
    }

    /// Build the current project
    pub fn build(&mut self) -> io::Result<()> {
        self.status_message = "Building project...".to_string();
        // Integration with compiler would go here
        Ok(())
    }

    /// Run the current project
    pub fn run_project(&mut self) -> io::Result<()> {
        self.status_message = "Running project...".to_string();
        // Integration with runtime would go here
        Ok(())
    }

    /// Format current file
    pub fn format(&mut self) -> io::Result<()> {
        self.status_message = "Formatting code...".to_string();
        // Integration with formatter would go here
        Ok(())
    }

    /// Toggle command palette
    pub fn toggle_command_palette(&mut self) {
        self.command_palette.visible = !self.command_palette.visible;
    }

    /// Toggle file browser
    pub fn toggle_file_browser(&mut self) {
        self.file_browser.visible = !self.file_browser.visible;
    }

    /// Quit the IDE
    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl FileBrowser {
    /// Create a new file browser
    pub fn new() -> Self {
        FileBrowser {
            current_dir: std::env::current_dir().unwrap_or_default(),
            files: Vec::new(),
            selected: 0,
            visible: false,
        }
    }

    /// Refresh file list
    pub fn refresh(&mut self) -> io::Result<()> {
        self.files.clear();
        for entry in std::fs::read_dir(&self.current_dir)? {
            let entry = entry?;
            self.files.push(entry.path());
        }
        Ok(())
    }

    /// Navigate up one directory
    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            let _ = self.refresh();
        }
    }

    /// Navigate into selected directory
    pub fn navigate_into(&mut self) {
        if let Some(path) = self.files.get(self.selected) {
            if path.is_dir() {
                self.current_dir = path.clone();
                let _ = self.refresh();
            }
        }
    }

    /// Get selected file
    pub fn get_selected(&self) -> Option<&PathBuf> {
        self.files.get(self.selected)
    }
}

impl CommandPalette {
    /// Create a new command palette
    pub fn new() -> Self {
        let commands = vec![
            Command {
                name: "Open File".to_string(),
                description: "Open a file for editing".to_string(),
                action: CommandAction::OpenFile,
            },
            Command {
                name: "Save File".to_string(),
                description: "Save the current file".to_string(),
                action: CommandAction::SaveFile,
            },
            Command {
                name: "Build".to_string(),
                description: "Build the current project".to_string(),
                action: CommandAction::Build,
            },
            Command {
                name: "Run".to_string(),
                description: "Run the current project".to_string(),
                action: CommandAction::Run,
            },
            Command {
                name: "Format".to_string(),
                description: "Format the current file".to_string(),
                action: CommandAction::Format,
            },
            Command {
                name: "Git Status".to_string(),
                description: "Show Git status".to_string(),
                action: CommandAction::GitStatus,
            },
            Command {
                name: "Quit".to_string(),
                description: "Exit the IDE".to_string(),
                action: CommandAction::Quit,
            },
        ];

        CommandPalette {
            query: String::new(),
            commands,
            selected: 0,
            visible: false,
        }
    }

    /// Filter commands by query
    pub fn filter_commands(&self) -> Vec<&Command> {
        if self.query.is_empty() {
            self.commands.iter().collect()
        } else {
            self.commands
                .iter()
                .filter(|cmd| {
                    cmd.name.to_lowercase().contains(&self.query.to_lowercase())
                        || cmd
                            .description
                            .to_lowercase()
                            .contains(&self.query.to_lowercase())
                })
                .collect()
        }
    }

    /// Get selected command
    pub fn get_selected(&self) -> Option<&Command> {
        let filtered = self.filter_commands();
        filtered.get(self.selected).copied()
    }
}

impl Default for GulTuiIde {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FileBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ide_creation() {
        let ide = GulTuiIde::new();
        assert!(ide.running);
        assert!(ide.current_file.is_none());
    }

    #[test]
    fn test_command_palette() {
        let palette = CommandPalette::new();
        assert!(!palette.visible);
        assert_eq!(palette.commands.len(), 7);
    }

    #[test]
    fn test_file_browser() {
        let browser = FileBrowser::new();
        assert!(!browser.visible);
    }
}
