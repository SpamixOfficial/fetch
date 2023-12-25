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

    let art_lines: Vec<&str> = art.split("\n").filter(|&x| !x.is_empty()).collect();
    if art_lines.len() > 8 {
        eprintln!("Error, height of ascii art is more than 8...");
        std::process::exit(1);
    };
    let longest_art_line = &art_lines.iter().max().unwrap().len();
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
    let height = 8;
    let wait = height/2 - ((art_lines.len() as f32 /2.0).ceil() as usize); 
    let mut wait_counter = wait.clone();
    //debug
    println!("{}", wait);
    let param_names = ["", "OS", "Arch", "Kernel", "Shell"];
    let mut i = 0;
   
    while i < param_names.len() {
        let mut tempstr = String::new();
        let mut ascii_str = String::new();
        let mut spaces_needed = longest_art_line.to_owned();
        if wait_counter > 0 {
            ascii_str.push_str(format!("{:>longest_art_line$}", "").as_str());
            wait_counter -= 1;
        } else if art_lines.len() < i-wait {
            ascii_str.push_str(format!("{:>longest_art_line$}", "").as_str());
        } else { 
            ascii_str = art_lines[i+1-wait].to_string();
            spaces_needed = longest_art_line-art_lines[i].len();
        };  

        tempstr.push_str("  ");
        tempstr.push_str(ascii_str.as_str());
        tempstr.push_str(format!("{:>spaces_needed$}  ", " ").as_str()); 
        if i == 0 {
            tempstr.push_str(format!("┏{:━>lastlength$}┓\n", "").as_str());
        }
        if param_names[i] != "" {
            let numspaces = &lastlength - &param_names[i].len() - 3; 
            tempstr.push_str(format!("┃ {}:{:>numspaces$} ┃\n", param_names[i], params[i]).as_str());
        } else { 
            tempstr.push_str(format!("┃{}┃\n", params[i]).as_str());
            if i == 0 {
                tempstr.push_str(format!("┣{:━>lastlength$}┫\n", "").as_str())
            }
        };
        if i == param_names.len()-1 {
            tempstr.push_str(format!("┗{:━>lastlength$}┛\n", "").as_str());
        };
        dbg!(&tempstr);
        outstr.push_str(tempstr.as_str());
        i += 1;
    }; 
/*
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
    }; */
    outstr
}

fn main() {
    let info = OsInfo::new();
    let art = get_ascii(&info);
    println!("{}", art);
    let output = create_output(art, info); 
    println!("{}", output);
}
