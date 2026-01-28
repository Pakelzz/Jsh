use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Layout}, style::{Color, Style, Stylize}, widgets::{Block, Borders, List, ListItem, ListState}};

use crate::{api::get_all_city, models::Kota, utils::{clear_line, spinner_loop}};

pub struct App {
    items: Vec<Kota>,
    state: ListState,
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
            state,
            exit: false,
        })
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = Layout::default()
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1)
            ])
            .split(area);

        let footer = Block::default()
            .bold()
        ;

        let items: Vec<ListItem> = self.items.iter().map(|f| {
            let new_text = format!("ID {}: {}", f.id, f.lokasi);
            ListItem::new(new_text)
        }).collect();

        let list = List::new(items)
            .block(Block::default().title(" List ").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, layout[0], &mut self.state);
        frame.render_widget(footer, layout[1]);

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
        match code {
            KeyCode::Down => self.next(),
            KeyCode::Up => self.prev(),
            KeyCode::Esc => self.exit = true,
            _ => {}
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
