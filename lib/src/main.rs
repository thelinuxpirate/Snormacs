use clap::Parser;
use std::{
		io,
		env,
		fs,
		process::Command,
};

/// Snormacs CLI Program written in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
		/// Number of installation mode
		#[arg(short, long, default_value_t = 0)]
		installer: u8,
}

fn program_check(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

fn main() {
    let args = Args::parse();

		// Installer Options
		if args.installer != 0 {
				let pkg: &str = "git";
				if program_check(pkg) {
						println!("(Git is installed!)\n");
				} else {
						println!("Please install \"git!\"");
				}

				let mut fetch_repo = Command::new("git");
				fetch_repo.args(&["clone", "https://github.com/thelinuxpirate/Snormacs", "temp/"]);
				
				match args.installer {
						1 => {
								println!("=+Snormacs Installer+=\nInstalling Snormacs;");
								println!("Fetching Snormacs...");								
								let _ = fetch_repo.output();
								println!("Done!");
								//Command::new("rm -r temp/");
						},

						2 => println!("V2 Install"),
						_ => println!("Invalid Version; Read the Man Page"),
				}
		}
}
