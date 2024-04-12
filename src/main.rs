use crossterm::style::Color;
use ratatui::{
	backend::CrosstermBackend,
	widgets::BorderType,
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
	handler::{
		handle_key_events,
		handle_mouse_events,
	},
	tui::Tui,
};

#[tokio::main]
async fn main() -> AppResult<()> {
	let mut app = App::new();

	//----------[ Command line arguments ]----------//
	let binding = clap_parse();

	let version: bool = *binding.get_one("version").unwrap();

	let color: i64 = *binding.get_one("color").unwrap();

	let binding = clap_parse();

	// Assuming clap_parse() parses the user input correctly
	let border_type_str: String = binding
		.get_one::<String>("BorderCorners")
		.unwrap()
		.to_string();
	let border_type: BorderType = match border_type_str.as_str() {
		"Plain" => BorderType::Plain,
		"Rounded" => BorderType::Rounded,
		_ => unreachable!(),
	};

	let level: i64 = *binding.get_one("startlevel").unwrap();

	if version {
		println!("tetrs v{}", env!("CARGO_PKG_VERSION"));
		std::process::exit(0);
	}
	app.level = level.try_into().unwrap();
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
			tui.draw(
				&mut app,
				Color::AnsiValue(color.try_into().unwrap()),
				border_type,
			)?;
		}
		//---------------------------------//

		//----------[ Event Handling ]----------//
		{
			match tui.events.next().await? {
				Event::Tick => app.tick(),
				Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
				Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app),
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
