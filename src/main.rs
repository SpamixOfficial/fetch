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
     
    outstr.push_str(format!("* {}@{}\n", info.username, info.hostname).as_str());
    outstr.push_str(format!("* OS:\t\t{}\n", info.os_type).as_str());
    outstr.push_str(format!("* Kernel:\t{}\n", info.os_release).as_str());
    outstr.push_str(format!("* ").as_str());
    return outstr;
}

fn main() {
    let info = OsInfo::new();
    let output = create_output(info);
    println!("{}", output);
}
