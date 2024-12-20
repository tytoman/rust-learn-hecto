use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyCode::Esc, KeyEvent, KeyModifiers};
mod ternimal;
use ternimal::Ternimal;

pub struct Editor {
    should_quit: bool
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Ternimal::initialize().unwrap();
        let result = self.repl();
        Ternimal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        print!("  {event:?}\r");
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Esc => {
                    self.should_quit = true;
                },
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
                
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Ternimal::clear_screen()?;
            print!("Goodbye!\r");
        } else {
            Self::draw_rows()?;
            Ternimal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Ternimal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }

}