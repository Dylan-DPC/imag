extern crate clap;
extern crate glob;
extern crate task_hookrs;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use std::process::exit;
use std::process::{Command, Stdio};

use libimagrt::runtime::Runtime;

mod ui;

use ui::build_ui;
fn main() {

    let name = "imag-todo";
    let version = &version!()[..];
    let about = "Interface with taskwarrior";
    let ui = build_ui(Runtime::get_default_cli_builder(name, version, about));
    
    let rt = {
        let rt = Runtime::new(ui);
        if rt.is_ok() {
            rt.unwrap()
        } else {
            println!("Could not set up Runtime");
            println!("{:?}", rt.unwrap_err());
            exit(1);
        }
    };

    

    let scmd = rt.cli().subcommand_name();
    match scmd {
        Some("tw-hook") => {
            let subcmd = rt.cli().subcommand_matches("tw-hook").unwrap();
            if subcmd.is_present("add") {
                println!("To be implemented");
                //
                // TODO @Kevin: import function aus task_hookrs benutzen, um
                //              stdin auszulesen, und dann auf dem
                //              task_hookrs::task::Task den Trait für die
                //              Umwandlung aufrufen.
                //
            }
            else if subcmd.is_present("delete") {
                println!("To be implemented");
                //
                // Functionality to delete Entry in the store
                //
            }
            else {
                // Should not be possible, as one argument is required via
                // ArgGroup
                panic!("Reached unreachable Code");
            }
        },
        Some("exec") => {
		let subcmd = rt.cli().subcommand_matches("exec").unwrap();
		let mut args = Vec::new();
		if let Some(exec_string) = subcmd.values_of("command") {
			for e in exec_string {
				args.push(e);
			}
			let mut tw_process = Command::new("/usr/local/bin/task").stdin(Stdio::null()).args(&args).spawn().unwrap_or_else(|e| {
				panic!("failed to execute taskwarrior: {}", e);
			});
			
			let output = tw_process.wait_with_output().unwrap_or_else(|e| {
				panic!("failed to unwrap output: {}", e);
			});
			let outstring = String::from_utf8(output.stdout).unwrap_or_else(|e| {
				panic!("failed to ececute: {}", e);
			});
			println!("{}", outstring);
		} else {
			panic!("faild to execute: You need to exec --command");
		}
            },
                _ => panic!("Reached unreachable Code"),
        }
    
}

