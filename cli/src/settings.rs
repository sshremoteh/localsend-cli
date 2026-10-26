use crate::storage::config::ResolvedConfig;
use crate::util;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use std::io::Stdout;
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SettingField {
    Alias,
    Port,
    Destination,
    AutoAccept,
    OrganizeFiles,
    Multicast,
    SaveHistory,
    SaveButton,
    CancelButton,
}

const FIELDS: [SettingField; 9] = [
    SettingField::Alias,
    SettingField::Port,
    SettingField::Destination,
    SettingField::AutoAccept,
    SettingField::OrganizeFiles,
    SettingField::Multicast,
    SettingField::SaveHistory,
    SettingField::SaveButton,
    SettingField::CancelButton,
];

pub enum SettingsOutcome {
    Open,
    Closed,
    Saved(ResolvedConfig),
}

pub struct Settings {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    config: ResolvedConfig,
    original_config: ResolvedConfig,
    selected_index: usize,
    editing: bool,
    input_buffer: String,
    status_message: Option<String>,
}

impl Settings {
    pub fn open(config: &ResolvedConfig) -> anyhow::Result<Self> {
        util::enter_alternate_screen()?;
        let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
        let mut settings = Self {
            terminal,
            config: config.clone(),
            original_config: config.clone(),
            selected_index: 0,
            editing: false,
            input_buffer: String::new(),
            status_message: None,
        };
        settings.draw();
        Ok(settings)
    }

    pub fn close(self) {
        util::leave_alternate_screen();
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> SettingsOutcome {
        if self.editing {
            match key.code {
                KeyCode::Enter => {
                    self.commit_edit();
                    self.editing = false;
                }
                KeyCode::Esc => {
                    self.editing = false;
                    self.status_message = Some("Editing cancelled.".to_string());
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                _ => {}
            }
            self.draw();
            return SettingsOutcome::Open;
        }

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                return SettingsOutcome::Closed;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                } else {
                    self.selected_index = FIELDS.len() - 1;
                }
                self.status_message = None;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_index + 1 < FIELDS.len() {
                    self.selected_index += 1;
                } else {
                    self.selected_index = 0;
                }
                self.status_message = None;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                return SettingsOutcome::Saved(self.config.clone());
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                match FIELDS[self.selected_index] {
                    SettingField::Alias => {
                        self.editing = true;
                        self.input_buffer = self.config.alias.clone();
                    }
                    SettingField::Port => {
                        self.editing = true;
                        self.input_buffer = self.config.port.to_string();
                    }
                    SettingField::Destination => {
                        self.editing = true;
                        self.input_buffer = self.config.destination.display().to_string();
                    }
                    SettingField::AutoAccept => {
                        self.config.auto_accept = match self.config.auto_accept.as_str() {
                            "none" => "paired".to_string(),
                            "paired" => "always".to_string(),
                            _ => "none".to_string(),
                        };
                    }
                    SettingField::OrganizeFiles => {
                        self.config.organize_files = match self.config.organize_files.as_str() {
                            "none" => "sender".to_string(),
                            "sender" => "date".to_string(),
                            _ => "none".to_string(),
                        };
                    }
                    SettingField::Multicast => {
                        self.config.multicast = !self.config.multicast;
                    }
                    SettingField::SaveHistory => {
                        self.config.save_history = !self.config.save_history;
                    }
                    SettingField::SaveButton => {
                        return SettingsOutcome::Saved(self.config.clone());
                    }
                    SettingField::CancelButton => {
                        return SettingsOutcome::Closed;
                    }
                }
            }
            _ => {}
        }

        self.draw();
        SettingsOutcome::Open
    }

    fn commit_edit(&mut self) {
        match FIELDS[self.selected_index] {
            SettingField::Alias => {
                let trimmed = self.input_buffer.trim();
                if !trimmed.is_empty() {
                    self.config.alias = trimmed.to_string();
                    self.status_message = Some(format!("Alias set to '{}'", self.config.alias));
                }
            }
            SettingField::Port => {
                if let Ok(port) = self.input_buffer.trim().parse::<u16>() {
                    if port > 0 {
                        self.config.port = port;
                        self.status_message = Some(format!("Port set to {}", port));
                    }
                } else {
                    self.status_message = Some("Invalid port number (1-65535)".to_string());
                }
            }
            SettingField::Destination => {
                let trimmed = self.input_buffer.trim();
                if !trimmed.is_empty() {
                    self.config.destination = PathBuf::from(trimmed);
                    self.status_message = Some(format!("Destination set to '{}'", trimmed));
                }
            }
            _ => {}
        }
    }

    fn draw(&mut self) {
        let _ = self.terminal.draw(|f| {
            let area = f.area();
            let modal_width = (area.width.saturating_sub(4)).min(72).max(40);
            let modal_height = (area.height.saturating_sub(2)).min(24).max(18);

            let x = (area.width.saturating_sub(modal_width)) / 2;
            let y = (area.height.saturating_sub(modal_height)) / 2;
            let modal_rect = Rect::new(x, y, modal_width, modal_height);

            f.render_widget(Clear, modal_rect);

            let block = Block::default()
                .title(" LocalSend Settings ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan));

            let inner = block.inner(modal_rect);
            f.render_widget(block, modal_rect);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),  // Title / spacer
                    Constraint::Min(12),   // Settings options list
                    Constraint::Length(2),  // Status / hint
                    Constraint::Length(2),  // Keyboard controls
                ])
                .split(inner);

            let mut lines = Vec::new();

            for (i, field) in FIELDS.iter().enumerate() {
                let is_selected = i == self.selected_index;
                let pointer = if is_selected { "▶ " } else { "  " };

                let style = if is_selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let val_style = Style::default().fg(Color::Green);

                match field {
                    SettingField::Alias => {
                        let val = if is_selected && self.editing {
                            format!("[ {}_ ]", self.input_buffer)
                        } else {
                            format!("[ {} ]", self.config.alias)
                        };
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}1. Device Alias:          "), style),
                            Span::styled(val, val_style),
                        ]));
                    }
                    SettingField::Port => {
                        let val = if is_selected && self.editing {
                            format!("[ {}_ ]", self.input_buffer)
                        } else {
                            format!("[ {} ]", self.config.port)
                        };
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}2. HTTP Server Port:      "), style),
                            Span::styled(val, val_style),
                        ]));
                    }
                    SettingField::Destination => {
                        let dest_str = self.config.destination.display().to_string();
                        let display_dest = if dest_str.len() > 28 {
                            format!("...{}", &dest_str[dest_str.len().saturating_sub(25)..])
                        } else {
                            dest_str
                        };
                        let val = if is_selected && self.editing {
                            format!("[ {}_ ]", self.input_buffer)
                        } else {
                            format!("[ {} ]", display_dest)
                        };
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}3. Destination Folder:    "), style),
                            Span::styled(val, val_style),
                        ]));
                    }
                    SettingField::AutoAccept => {
                        let label = match self.config.auto_accept.as_str() {
                            "paired" => "Paired Devices Only",
                            "always" => "Everyone (Always)",
                            _ => "Off (Always Ask)",
                        };
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}4. Auto-Accept Transfers: "), style),
                            Span::styled(format!("[ {label} ]"), val_style),
                        ]));
                    }
                    SettingField::OrganizeFiles => {
                        let label = match self.config.organize_files.as_str() {
                            "sender" => "By Sender Device",
                            "date" => "By Date (YYYY-MM-DD)",
                            _ => "Flat (Disabled)",
                        };
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}5. Organize Downloads:   "), style),
                            Span::styled(format!("[ {label} ]"), val_style),
                        ]));
                    }
                    SettingField::Multicast => {
                        let label = if self.config.multicast { "Active" } else { "Stealth (Off)" };
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}6. Multicast Discovery:  "), style),
                            Span::styled(format!("[ {label} ]"), val_style),
                        ]));
                    }
                    SettingField::SaveHistory => {
                        let label = if self.config.save_history { "Enabled" } else { "Disabled" };
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}7. Transfer History:     "), style),
                            Span::styled(format!("[ {label} ]"), val_style),
                        ]));
                    }
                    SettingField::SaveButton => {
                        lines.push(Line::from(""));
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}   [ SAVE AND APPLY SETTINGS ]"), if is_selected {
                                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
                            } else {
                                Style::default().fg(Color::Green)
                            }),
                        ]));
                    }
                    SettingField::CancelButton => {
                        lines.push(Line::from(vec![
                            Span::styled(format!("{pointer}   [ Cancel / Return ]"), if is_selected {
                                Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)
                            } else {
                                Style::default().fg(Color::DarkGray)
                            }),
                        ]));
                    }
                }
            }

            let list_widget = Paragraph::new(lines);
            f.render_widget(list_widget, chunks[1]);

            // Status message
            if let Some(msg) = &self.status_message {
                let status_widget = Paragraph::new(Span::styled(msg, Style::default().fg(Color::Magenta)));
                f.render_widget(status_widget, chunks[2]);
            } else if self.editing {
                let edit_widget = Paragraph::new(Span::styled("Type new value and press [Enter] to apply, [Esc] to cancel", Style::default().fg(Color::Yellow)));
                f.render_widget(edit_widget, chunks[2]);
            }

            // Controls footer
            let footer = Line::from(vec![
                Span::styled("↑/↓: ", Style::default().fg(Color::DarkGray)),
                Span::raw("Navigate  "),
                Span::styled("Enter/Space: ", Style::default().fg(Color::DarkGray)),
                Span::raw("Edit/Toggle  "),
                Span::styled("S: ", Style::default().fg(Color::DarkGray)),
                Span::raw("Save  "),
                Span::styled("Esc: ", Style::default().fg(Color::DarkGray)),
                Span::raw("Back"),
            ]);
            f.render_widget(Paragraph::new(footer), chunks[3]);
        });
    }
}
