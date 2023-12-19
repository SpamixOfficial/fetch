use std::env;
use std::process::exit;
use uname::uname;

struct OsInfo {
    os_type: String,
    os_arch: String,
    shell: String,
    username: String,
    os_release: String,
    hostname: String,
}

impl os_info {
    fn new() -> Self {
        os_type = env::consts::OS;

        os_arch = env::consts::ARCH;
        shell = match env::var_os("SHELL") {
            Some(v) => v.into_string().unwrap(),
            None => String::from("Unknown")
        };
        username = match env::var_os("LOGNAME") {
            Some(v) => v.into_string().unwrap(),
            None => String::from("unknown")
        };
        os_release = uname().unwrap().release;
        hostname = uname().unwrap().nodename; 
    }
}

fn get_os_info() -> [String; 6] {
    let os_type = env::consts::OS;

    if os_type == "windows" {
        println!("Windows is not supported at the moment.");
        exit(1);
    };
    let os_arch = env::consts::ARCH;
    let shell = match env::var_os("SHELL") {
        Some(v) => v.into_string().unwrap(),
        None => String::from("Unknown")
    };
    let username = match env::var_os("LOGNAME") {
        Some(v) => v.into_string().unwrap(),
        None => String::from("none")
    };
    let os_release = uname().unwrap().release;
    let hostname = uname().unwrap().nodename;
    return [os_type.to_string(), os_arch.to_string(), shell, username, os_release, hostname]
}



fn main() {
    let info = ;
    
}
