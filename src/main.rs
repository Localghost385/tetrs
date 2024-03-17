use crossterm::style::Color;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tetrs::app::{App, AppResult};
use tetrs::clap::clap_parse;
use tetrs::event::{Event, EventHandler};
use tetrs::handler::handle_key_events;
use tetrs::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();

    //----------[ Command line arguments ]----------//
    let binding = clap_parse();
    let args: i64 = *binding.get_one("color").unwrap();
    //----------------------------------------------//

    //----------[ Init UI ]----------//
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(50);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    //-------------------------------//

    //----------[ Event Loop ]----------//
    while app.running {
        //----------[ Rendering ]----------//
        {
            tui.draw(&mut app, Color::AnsiValue(args.try_into().unwrap()))?;
        }
        //---------------------------------//

        //----------[ Event Handling ]----------//
        {
            match tui.events.next().await? {
                Event::Tick => app.tick(),
                Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            }
        }
        //--------------------------------------//
    }
    //----------------------------------//

    //----------[ Cleanup ]----------//
    {
        tui.exit()?;
        Ok(())
    }
    //-------------------------------//
}
