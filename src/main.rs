use colored::Colorize;
use std::env;

mod tram;
mod tramtype;

use tramtype::{TramFlag, TramCommand, TramErr};

fn main() {
	if let Err(e) = tram_command() {
		print_err(&e);
		return;
	}
}

fn tram_command() -> Result<(), TramErr> {
	let mut argv: Vec<TramFlag> = Vec::with_capacity(10);
	let mut args_iter = env::args();
	let mut command: TramCommand = TramCommand::Undetermined;

	if let Some(arg) = args_iter.nth(1) {
		command = match arg.as_str() {
			"list" => TramCommand::Tram,
			"edit" => TramCommand::Tram,
			"open" => TramCommand::Tram,
			"close" => TramCommand::Tram,
			"init" => TramCommand::Tram,
			"config" => TramCommand::Tram,
			"ding" | "cink" => TramCommand::Tram,
			arg => {
				argv.push(parse_flag(arg, &0));
				TramCommand::Undetermined
			}
		}
	} else {
		return Err(TramErr::CommandMissing);
	}

	for (i, arg) in args_iter.enumerate() {
			argv.push(parse_flag(arg.as_str(), &i));
	}
	println!("{}, {:#?}", argv.len(), argv);
	Ok(())
}

fn parse_flag(arg: &str, i: &usize) -> TramFlag {
	if &arg[..1] == "-" {
		/*if &arg[1..1] == "-" { // 
			match &arg[2..] {
				_ => return TramFlag::Arg(*i as u8),
			}
		}*/
	}
	TramFlag::Arg(*i as u8 + 1)
}

fn print_err(e: &TramErr) {
	match e {
		TramErr::CommandMissing => {
			println!("{} no subcommand specified", "Fatal:".red());
			println!("	Hint: use `-h --subcommands` for a list of subcommands");
		}
		TramErr::ParsingFailed => {
			println!("{} argument parsing failed", "Fatal:".red());
		}
		TramErr::Generic(m) => {
			println!("{} {}", "Fatal:".red(), m);
		}
	}
}

fn print_test_issue() {
	println!("\n{} {}", "[ 4856 ]".green().bold(), "( Bug )".red().bold());
	println!(
		"{}",
		"Windows Terminal supports truecolor, but colored doesn't detect that"
			.green()
			.bold()
	);
	println!(
		"{} {}",
		"Group A".cyan(),
		"(Physics) (Rendering) (WebGPU)".italic().blue()
	);
}

const EASTER_EGG: &'static str = r#"
-------------------------,',-----------------------------------,',--------
         _---_          ', ,'                 _---_           ', ,'
  ,-------`-´-------------'-------.     ,------`-´--------------'-------.
  /''|```|```|```|```|```|```|``|` |    /''|```|```|```|```|```|```|``|``|
 |---'---'---'---'---'---'---'--'--|   |---'---'---'---'---'---'---'--'--|
 ,_    ______           ______     |=-=,_    ______           ______     |
  '---'(O)(O)'---------'(O)(O)'---'     '---'(O)(O)'---------'(O)(O)'---'
``````````````````````````````````````````````````````````````````````````
 "#;
