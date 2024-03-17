use clap::*;

//----------[ Args ]----------//
pub fn clap_parse() -> ArgMatches {
    //----------[ Color  ]----------//
    let cmd = clap::Command::new("cargo").bin_name("tetrs").arg(
        Arg::new("color")
            .long("color")
            .short('c')
            .value_parser(0..=15)
            .default_value("3"),
    );
    //------------------------------//

    cmd.get_matches()
}
//----------------------------//
