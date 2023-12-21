use std::{
    env,
    fs,
    path
};
use uname::uname;

struct OsInfo {
    os_type: String,
    os_arch: String,
    shell: String,
    username: String,
    os_release: String,
    hostname: String,
}

impl OsInfo {
    fn new() -> Self {
        Self {
            os_type: env::consts::OS.to_string(),

            os_arch: env::consts::ARCH.to_string(),
            shell: match env::var_os("SHELL") {
                Some(v) => v.into_string().unwrap(),
                None => String::from("Unknown"),
            },
            username: match env::var_os("LOGNAME") {
                Some(v) => v.into_string().unwrap(),
                None => String::from("unknown"),
            },
            os_release: uname().unwrap().release,
            hostname: uname().unwrap().nodename,
        }
    }
}

fn get_ascii(info: &OsInfo) -> String {
    let std_linux_art = "
  .~.
  /V\\
 // \\\\
/(   )\\
 ^`~'^
";
    let std_unknown_art = "
__
 _)
|
*
";
    let art: String;
    if &info.os_type == "linux" {
        if path::Path::exists(path::Path::new("/etc/ascii-art")) {
            art = fs::read_to_string("/etc/ascii-art").unwrap();
        } else {
            art = std_linux_art.to_string();
        };
    } else {
        art = std_unknown_art.to_string();
    }; 
    art
}

fn create_output(art: String, info: OsInfo) -> String {
    let mut outstr = String::new();

    let art_lines = art.split("\n");
    if art_lines.len() > 8 {
        eprintln!("Error, height of ascii art is more than 8...");
        std::process::exit(1);
    };
    let user_host = format!("{}@{}", info.username, info.hostname);
    let os = &info.os_type;
    let arch = &info.os_arch;
    let kernel = &info.os_release;
    let shell = &info.shell;

    let params = [&user_host, &os, &arch, &kernel, &shell];
    let mut lastlength = 0;
    for param in params {
        let length = param.len(); 
        if length > lastlength {
            lastlength = length;
        };
    };
    
    let height: u8 = 8;
    let wait = ...; 
    let param_names = ["", "OS", "Arch", "Kernel", "Shell"];
    let mut i = 0;
    while i < param_names.len() {
        if i == 0 {
            outstr.push_str(format!("┏{:━>lastlength$}┓\n", "").as_str());
        }
        if param_names[i] != "" {
            let numspaces = &lastlength - &param_names[i].len() - 3; 
            outstr.push_str(format!("┃ {}:{:>numspaces$} ┃\n", param_names[i], params[i]).as_str());
        } else { 
            outstr.push_str(format!("┃{}┃\n", params[i]).as_str());
            if i == 0 {
                outstr.push_str(format!("┣{:━>lastlength$}┫\n", "").as_str())
            }
        };
        if i == param_names.len()-1 {
            outstr.push_str(format!("┗{:━>lastlength$}┛\n", "").as_str());
        }
        i += 1;
    };
    outstr
}

fn main() {
    let info = OsInfo::new();
    let art = get_ascii(&info);
    println!("{}", art);
    let output = create_output(art, info); 
    println!("{}", output);
}
