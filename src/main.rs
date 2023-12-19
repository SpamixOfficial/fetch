use std::env;
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

fn get_ascii() {
    println!("Placeholder");
}

fn create_output(info: OsInfo) -> String {
    let mut outstr = String::new();

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
    }
    
    let param_names = ["", "OS", "Arch", "Kernel", "Shell"];
    let mut i = 0;
    while i < param_names.len() {
        if param_names[i] != "" {
            println!("* {}: {}", param_names[i], params[i]);
        } else {
            println!("{}", params[i]);
        };
        i += 1;
    };
    /*outstr.push_str(format!("* {:X>5}\n", &user_host).as_str());
    outstr.push_str(format!("* OS:\t\t{}\n", info.os_type).as_str());
    outstr.push_str(format!("* Kernel:\t{}\n", info.os_release).as_str());
    outstr.push_str(format!("* ").as_str());*/
    return outstr;
}

fn main() {
    let info = OsInfo::new();
    let output = create_output(info);
    println!("{}", output);
}
