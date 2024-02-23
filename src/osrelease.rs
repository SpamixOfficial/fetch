use serde::Deserialize;
use nix::sys::utsname;
use std::{
    collections::HashMap,
    env, fs,
    io::Read,
    path,
    process::exit,
    str,
};



#[cfg(target_os = "macos")]
use plist;


#[derive(Debug, Clone)]
pub struct OsRelease {
    // Old value I might bring back
    //exists: bool,
    pub os_release: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
struct SystemVersionPlist {
    //Unnecessary field which I might bring back!
    //#[serde(rename(deserialize = "ProductBuildVersion"))]
    //build: String,
    #[serde(rename(deserialize = "ProductName"))]
    name: String,
    #[serde(rename(deserialize = "ProductVersion"))]
    version: String,
}

#[derive(Debug, Clone)]
pub struct OsInfo {
    pub os_release_file_content: OsRelease,
    pub os_type: String,
    pub os_arch: String,
    pub shell: String,
    pub username: String,
    pub os_release: String,
    pub hostname: String,
}

impl OsRelease {
    pub fn new() -> OsRelease {
        let mut os_release_values: HashMap<String, String> = HashMap::new();
        // check if the file exists
        let os_release_file_path = if !cfg!(target_os = "macos") {
            path::Path::new("/etc/os-release")
        } else {
            path::Path::new("/System/Library/CoreServices/SystemVersion.plist")
            //path::Path::new("demo.plist")
        };
        let os_release_file_exists = match path::Path::try_exists(os_release_file_path) {
            Ok(_) => true,
            Err(_) => false,
        };

        // If file exists, read the contents
        if os_release_file_exists && !cfg!(target_os = "macos") {
            // Read the file to the variable raw_file_content
            let mut raw_file_content = String::new();
            let mut raw_file = fs::File::open(os_release_file_path).unwrap();
            raw_file.read_to_string(&mut raw_file_content).unwrap();

            // Split file content into lines and convert to String
            let raw_file_lines: Vec<String> = raw_file_content
                .lines()
                .map(|value| value.to_string())
                .collect();
            // Insert every line into the hashmap
            raw_file_lines.iter().for_each(|line| {
                let values: Vec<String> = line
                    .splitn(2, "=")
                    .map(|value| value.replacen("\"", "", 2))
                    .collect();
                // If less than 2 values are present, we can assume something is wrong with that
                // line and skip it
                if values.len() == 2 {
                    os_release_values.insert(
                        values.get(0).unwrap().to_string(),
                        values.get(1).unwrap().to_string(),
                    );
                };
            });
        } else if os_release_file_exists && cfg!(target_os = "macos") { 
            let systemplist: SystemVersionPlist = plist::from_file(os_release_file_path).expect(
                "SystemVersion.Plist is present but not readable. Something is seriously wrong!",
            );
            os_release_values.insert("VERSION_ID".to_string(), systemplist.version);
            os_release_values.insert("PRETTY_NAME".to_string(), systemplist.name);
            os_release_values.insert("ID".to_string(), env::consts::OS.to_string());
        } else {
            eprintln!("No os-release-type file present. Exiting with code 1");
            exit(1);
        };
        OsRelease {
            //exists: os_release_file_exists,
            os_release: os_release_values,
        }
    }
}

impl OsInfo {
    pub fn new() -> Self {
        let uname = utsname::uname().unwrap();
        Self {
            os_release_file_content: OsRelease::new(),
            os_type: env::consts::OS.to_string(),
            os_arch: env::consts::ARCH.to_string(),
            shell: match env::var_os("SHELL") {
                Some(v) => v.into_string().unwrap(),
                None => String::from("Unknown"),
            },
            username: match env::var_os("LOGNAME") {
                Some(v) => v.into_string().unwrap(),
                None => String::from("Unknown"),
            },
            os_release: String::from(uname.release().to_str().unwrap()),
            hostname: String::from(uname.nodename().to_str().unwrap()),
        }
    }
}
