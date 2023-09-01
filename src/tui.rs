use std::io::{self, Stdout};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::Layout,
    prelude::{Backend, Constraint, CrosstermBackend, Direction},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(Debug)]
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self, io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        self.prolog()?;

        let mut should_run = true;
        let mut prompt = Input::new(String::new());

        while should_run {
            self.terminal.draw(|frame| ui(frame, &prompt))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => should_run = false,
                    _ => {
                        prompt.handle_event(&Event::Key(key));
                    }
                }
            }
        }

        self.epilog()?;
        Ok(())
    }

    fn prolog(&mut self) -> Result<(), io::Error> {
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;
        Ok(())
    }

    fn epilog(&mut self) -> Result<(), io::Error> {
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        disable_raw_mode()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, prompt: &Input) {
    let main_ui = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(frame.size());

    let input_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
        .split(main_ui[0]);

    let results = Block::default().title("Files").borders(Borders::ALL);
    let input_field =
        Paragraph::new(prompt.value()).block(Block::default().title("Input").borders(Borders::ALL));
    let file_content = Block::default().title("File content").borders(Borders::ALL);

    frame.render_widget(results, input_chunk[0]);
    frame.render_widget(input_field, input_chunk[1]);
    frame.render_widget(file_content, main_ui[1]);
}
