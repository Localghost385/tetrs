use crossterm::style::Color;
use ratatui::{
	backend::CrosstermBackend,
	Terminal,
};
use std::io;
use tetrs::{
	app::{
		App,
		AppResult,
	},
	clap::clap_parse,
	event::{
		Event,
		EventHandler,
	},
	handler::handle_key_events,
	tui::Tui,
};

#[tokio::main]
async fn main() -> AppResult<()> {
	let mut app = App::new();

	//----------[ Command line arguments ]----------//
	let binding = clap_parse();
	let version: bool = *binding.get_one("version").unwrap();
	let color: i64 = *binding.get_one("color").unwrap();
	let tick_count_target: i64 = *binding.get_one("TickCountTarget").unwrap();

    if version {
        println!("tetrs v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
	app.default_tick_count_target = tick_count_target.try_into().unwrap();
	app.default_tick_count_target += 2;
	//----------------------------------------------//

	//----------[ Init UI ]----------//
	let backend = CrosstermBackend::new(io::stderr());
	let terminal = Terminal::new(backend)?;
	let events = EventHandler::new(30);
	let mut tui = Tui::new(terminal, events);
	tui.init()?;
	//-------------------------------//

	//----------[ Event Loop ]----------//
	while app.running {
		//----------[ Rendering ]----------//
		{
			tui.draw(&mut app, Color::AnsiValue(color.try_into().unwrap()))?;
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
