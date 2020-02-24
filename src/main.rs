use std::{env, error};
use clap::{Arg, App, SubCommand};

use markdownizer;

// use parser::hello_parser;

fn main() -> Result<(), Box<dyn error::Error>> {
    let app = App::new("Markdownizer")
        .version("1.0")
        .author("Henri Bourcereau <henri@bourcereau.fr>")
        .about("A markdown based GTD system manager")
        .arg(Arg::with_name("directory")
             .short("d")
             .long("directory")
             .value_name("ROOT")
             .help("Directory path of the markdown files")
             .takes_value(true))
        .arg(Arg::with_name("relative_from")
             .short("r")
             .long("relative-to")
             .value_name("RELATIVETO")
             .help("Path from which display relative paths")
             .takes_value(true))
        .subcommand(SubCommand::with_name("list")
                    .about("show projects"));

    let matches = app.get_matches();
    let current = get_current_dir();
    let root = matches.value_of("directory").unwrap_or(&current);
    let relative_from = matches.value_of("relative_from").unwrap_or(&current);

	// println!("{:?}", hello_parser("hello world"));
	// println!("{:?}", hello_parser("goodbye hello again"));
    // println!("Value for directory: {}", root);
    //
    let proot = std::path::PathBuf::from(root);
    let markdownizer = markdownizer::Markdownizer::new(&proot);

    // let markdownizer = markdownizer::Markdownizer::new(root);
    let plist = markdownizer.project_list().unwrap();
    for entry in plist {
        match entry {
            project => println!("({}) ({})", project.title, project.tasks.len()),
            e => println!("Not a project: {:?}", e)
        }
    }

    Ok(())
}

fn get_current_dir() -> String {
    env::current_dir()
    .map( |cd|
          String::from(cd.as_path().to_str().unwrap())
    ).expect("Can't find current path")
}
