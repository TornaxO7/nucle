use std::{
    io::{self, Stdout},
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::Layout,
    prelude::{Backend, Constraint, CrosstermBackend, Direction},
    widgets::{Block, Borders},
    Frame, Terminal,
};

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

        self.terminal.draw(|frame| {
            let main_ui = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(frame.size());

            let input_chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(main_ui[0]);

            let results = Block::default().title("Files").borders(Borders::ALL);
            let input_field = Block::default().title("Input").borders(Borders::ALL);
            let file_content = Block::default().title("File content").borders(Borders::ALL);

            frame.render_widget(results, input_chunk[0]);
            frame.render_widget(input_field, input_chunk[1]);
            frame.render_widget(file_content, main_ui[1]);
        })?;
        thread::sleep(Duration::from_secs(5));

        self.epilog()?;
        Ok(())
    }

    fn prolog(&mut self) -> Result<(), io::Error> {
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    fn epilog(&mut self) -> Result<(), io::Error> {
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
