use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

fn main() {
    //Get params
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let debug = args.len() > 0 && ( args.first().unwrap().eq("--debug") || args.first().unwrap().eq("-d") ); 
    if debug { 
        args.remove(0); //remove --debug
        println!("[WRAPPER] Params: {:?}", args); 
    }

    //Get link filename
    let file_path: PathBuf = env::current_exe().unwrap();

    let mut file_path_manifest_str: OsString = file_path.into_os_string();
    file_path_manifest_str.push(".link");
    if debug { println!("[WRAPPER] Config file: {}", file_path_manifest_str.to_str().unwrap()); }

    //Get link content
    let command_line: String = fs::read_to_string(file_path_manifest_str)
        .expect("[WRAPPER ERROR] Do not find the config file");
    if debug { println!("[WRAPPER] Link to: {}", command_line.trim()); }

    //Launch exe
    let mut command = Command::new(command_line.trim());
    command.args(args);

    let mut child = command.spawn()
        .expect("[WRAPPER ERROR] Can't start command");
    if debug { println!("[WRAPPER] Process ID: {}", child.id()); }

    let code = child.wait()
        .expect("[WRAPPER ERROR] Fail to wait child");

    //Exit with status code
    let code = code.code().unwrap();
    if debug { println!("[WRAPPER] Status code: {}", code); }
    process::exit(code);
}
