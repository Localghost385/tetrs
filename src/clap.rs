use clap::*;

//----------[ Args ]----------//
pub fn clap_parse() -> ArgMatches {
	let cmd = clap::Command::new("cargo")
		.bin_name("tetrs")
		.about("A tui Tetris game written in Rust!")
		//----------[ Version ]----------//
		.arg(
			Arg::new("version")
			.long("version")
			.short('v')
			.action(ArgAction::SetTrue),
		)
		//-------------------------------//
		//----------[ Color ]----------//
		.arg(
			Arg::new("color")
				.long("color")
				.short('c')
				.value_parser(0..=15)
				.default_value("3"),
		)
		//-----------------------------//
		//----------[ Fall Speed ]----------//
		.arg(
			Arg::new("TickCountTarget")
				.long("tickcounttarget")
				.short('t')
				.value_parser(1..=15)
				.default_value("15"),
		);
	//----------------------------------//

	cmd.get_matches()
}
//----------------------------//
