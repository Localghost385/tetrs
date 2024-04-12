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
		//----------[ Border Corners ]----------//
		.arg(
			Arg::new("BorderCorners")
				.long("border")
				.short('b')
				.value_parser(|input: &str| {
					if input == "Plain" || input == "Rounded" {
						Ok(input.to_string())
					} else {
						Err("Invalid value for BorderCorners. Allowed values are 'Plain' or 'Rounded'".to_string())
					}
				})
				.default_value("Plain"),
		)
		//--------------------------------------//
		//----------[ Start Level ]----------//
		.arg(
			Arg::new("startlevel")
				.long("level")
				.short('l')
				.value_parser(1..=15)
				.default_value("1"),
		)
		//----------------------------------//
		//----------[ Buttons ]----------//
		.arg(
			Arg::new("ControlButtons")
				.long("buttons")
				.short('B')
				//process bool input
				.value_parser(|input: &str| match input.to_lowercase().as_str() {
					"true" => Ok(true),
					"false" => Ok(false),
					_ => Err(
						"Invalid value for bool. Allowed values are 'true' or 'false'".to_string(),
					),
				})
				.default_value("false"),
		);
	//-------------------------------//

	cmd.get_matches()
}
//----------------------------//
