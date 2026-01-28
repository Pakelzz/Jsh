use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, HorizontalAlignment, Layout}, style::{Color, Style}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Borders, List, ListItem, ListState, Paragraph}};
use tokio::sync::mpsc::{Receiver, error::TryRecvError};

use crate::{api::{get_all_city, get_jadwal2}, models::{Jadwal, JadwalResponse, Kota}, time, utils::{clear_line, spinner_loop}};

pub enum Commands {
    Normal,
    Search
}

pub struct App {
    items: Vec<Kota>, // The main data works with Commands
    filter: Vec<Kota>, // For filter items state works with Commands
    jadwal_rx: Option<Receiver<JadwalResponse>>,
    jadwal: Option<Jadwal>,
    enter: bool,
    state: ListState,
    command: Commands,
    input: String,
    exit: bool,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let result = tokio::select! {
            res = get_all_city() => res,
            _ = spinner_loop("Loading list all possible city ") => unreachable!(),
        };

        clear_line(0, 0);

        let mut items = Vec::new();
        let mut state = ListState::default();
        state.select_first();

        match result {
            Ok(res) => {
                if let Some(i) = res.data {
                    items = i;
                }
            }
            Err(e) => return Err(e),
        }

        Ok(Self {
            items,
            filter: Vec::new(),
            jadwal_rx: None,
            jadwal: None,
            enter: false,
            state,
            command: Commands::Normal,
            input: String::new(),
            exit: false,
        })
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if !self.enter {
            let layout = Layout::default()
            .constraints([
                Constraint::Min(0),
                Constraint::Length(2)
            ])
            .split(area);

            let items: Vec<ListItem> = if self.filter.is_empty() {
                self.items.iter().map(|f| {
                    let list_city = format!("ID {}: {}", f.id, f.lokasi);
                    ListItem::new(list_city)
                }).collect()
            } else {
                self.filter.iter().map(|f| {
                    let list_city = format!("ID {}: {}", f.id, f.lokasi);
                    ListItem::new(list_city)
                }).collect()
            };

            let list = List::new(items)
                .block(Block::default()
                .title(" List ")
                .borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
                .highlight_symbol("> ");

            frame.render_stateful_widget(list, layout[0], &mut self.state);
            // frame.render_widget(footer, layout[1]);

            match self.command {
                Commands::Normal => {},
                Commands::Search => {
                    let text = Text::from(
                        Line::from(vec![
                            Span::styled(
                                " Search: ",
                                Style::default()
                                    .fg(Color::Yellow)
                            ),
                            Span::styled(
                                self.input.as_str(),
                                Style::default()
                                    .fg(Color::Yellow)
                            )
                        ])
                    );

                    let footer = Paragraph::new(text)
                        .block(Block::default().borders(Borders::NONE));

                    frame.render_widget(footer, layout[1]);
                }
            }
        } else {
            let layout = Layout::default()
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0)
                ])
                .split(area);
            
            // jd for make layout_jadwal looks better
            let jd = Layout::horizontal([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33)
                ])
                .split(layout[1]);

            let layout_time = Layout::vertical([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(15),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
                .margin(2)
                .split(jd[0]);

            let layout_jadwal = Layout::vertical([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(15),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
                .margin(2)
                .split(jd[1]);

            if let Some(rx) = &mut self.jadwal_rx {
                match rx.try_recv() {
                    Ok(x) => {
                        self.jadwal = x.data;
                    },
                    Err(TryRecvError::Empty) => {},
                    Err(TryRecvError::Disconnected) => self.jadwal_rx = None
                }
            }

            if let Some(jadwal) = &self.jadwal {
                frame.render_widget(
                    Paragraph::new(jadwal.lokasi.clone())
                    .alignment(HorizontalAlignment::Center)
                    .block(Block::default().borders(Borders::ALL)),
                    layout[0]
                );

                frame.render_widget(
                    Paragraph::new(jadwal.jadwal.subuh.clone())
                    .alignment(HorizontalAlignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(border::PLAIN)
                            .title(" Subuh ")
                            .title_alignment(HorizontalAlignment::Center)),
                    layout_jadwal[0]
                );

                frame.render_widget(
                    Paragraph::new(jadwal.jadwal.dzuhur.clone())
                    .alignment(HorizontalAlignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(border::PLAIN)
                            .title(" Dzuhur ")
                            .title_alignment(HorizontalAlignment::Center)),
                    layout_jadwal[1]
                );

                frame.render_widget(
                    Paragraph::new(jadwal.jadwal.ashar.clone())
                    .alignment(HorizontalAlignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(border::PLAIN)
                            .title(" Ashar ")
                            .title_alignment(HorizontalAlignment::Center)),
                    layout_jadwal[2]
                );
                
                frame.render_widget(
                    Paragraph::new(jadwal.jadwal.maghrib.clone())
                    .alignment(HorizontalAlignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(border::PLAIN)
                            .title(" Maghrib ")
                            .title_alignment(HorizontalAlignment::Center)),
                    layout_jadwal[3]
                );
                
                frame.render_widget(
                    Paragraph::new(jadwal.jadwal.isya.clone())
                    .alignment(HorizontalAlignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(border::PLAIN)
                            .title(" Isya ")
                            .title_alignment(HorizontalAlignment::Center)),
                    layout_jadwal[4]
                );

                frame.render_widget(
                    Paragraph::new(time::get_time())
                    .alignment(HorizontalAlignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(border::PLAIN)
                            .title(" Time ")
                            .title_bottom(" ESC : Close ")
                            .title_alignment(HorizontalAlignment::Center)
                        ),
                    layout_time[2]
                )


            } else {
                frame.render_widget(
                    Paragraph::new("Loading ..."), 
                    layout[0]
                );
            }

        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(x) => {
                if x >= self.items.len() - 1 {
                    0
                } else {
                    x + 1
                }
            }
            None => 0
        };

        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(x) => {
                if x == 0 {
                    self.items.len() - 1
                } else {
                    x - 1
                }
            }
            None => 0
        };

        self.state.select(Some(i));
    }

    fn handle_key(&mut self, code: KeyCode) {
        match self.command {
            Commands::Normal => {
                match code {
                    KeyCode::Down => self.next(),
                    KeyCode::Up => self.prev(),
                    KeyCode::Char(c) => {
                        if c == '/' {
                            self.command = Commands::Search;
                        }
                    },
                    KeyCode::Esc => self.exit = true,
                    KeyCode::Enter => {
                        if let Some(i) = self.state.selected() {
                            if let Some(j) = self.items.get(i) {
                                let (tx, rx) = tokio::sync::mpsc::channel(1);
                                self.jadwal_rx = Some(rx);
                                self.enter = true;

                                let id = j.id.clone();

                                tokio::spawn(async move {
                                    if let Ok(x) = get_jadwal2(id, time::now()).await {
                                        let _ = tx.send(x).await;
                                    }
                                });
                            } 
                        }
                    },
                    _ => {}
                }
            },
            Commands::Search => {
                match code {
                    KeyCode::Char(c) => {
                        self.input.push(c);
                        self.filter = self.items.iter().filter(|f| f.lokasi.to_lowercase().contains(&self.input.to_lowercase())).cloned().collect();
                    },
                    KeyCode::Esc => {
                        if self.enter {
                            self.exit = true;
                            self.filter.clear();
                        } else {
                            self.command = Commands::Normal;
                            self.filter.clear();
                        }
                    },
                    KeyCode::Backspace => {
                        self.input.pop();
                        self.filter = self.items.iter().filter(|f| f.lokasi.to_lowercase().contains(&self.input.to_lowercase())).cloned().collect();
                    },
                    KeyCode::Down => self.next(),
                    KeyCode::Up => self.prev(),
                    KeyCode::Enter => {
                        if let Some(i) = self.state.selected() {
                            if let Some(j) = self.filter.get(i) {
                                let (tx, rx) = tokio::sync::mpsc::channel(1);
                                self.jadwal_rx = Some(rx);
                                self.enter = true;

                                let id = j.id.clone();

                                tokio::spawn(async move {
                                    if let Ok(x) = get_jadwal2(id, time::now()).await {
                                        let _ = tx.send(x).await;
                                    }
                                });
                            } 
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key(key.code);
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

}
