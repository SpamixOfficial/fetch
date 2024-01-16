use dirs;
use nix::sys::utsname;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    io::Read,
    path,
    process::{exit, Command},
    str,
};
use taap;
use toml;

fn main() {
    // Argument creation and handling
    let mut arguments = taap::Argument::new(
        "fetch",
        "Minimal and easy fetch tool written in rust",
        "",
        "SpamixOfficial 2024",
    );
    arguments.add_option('c', "config", "1", Some("Manually specify the config file"));
    arguments.add_option('-', "os-logo", "1", Some("Manually specify OS logo"));
    // todo
    //arguments.add_option('l', "list-art", "0", Some("List all known available art"));
    arguments.add_exit_status(0, "Everything went well");
    arguments.add_exit_status(1, "An error occurred");
    let args = arguments.parse_args();

    // Start of program
    let info = OsInfo::new();
    let config = Config::get_config(&info, args.get("c").unwrap().to_owned());

    let os_logo = args.get("os-logo").unwrap();
    let art;
    if os_logo.0 {
        art = get_ascii(&info, Some(os_logo.1.get(0).unwrap().to_owned()), &config);
    } else {
        art = get_ascii(&info, None, &config);
    };

    let output = create_output(art, info, config.modules, config.display);
    println!("{}", output);
}

#[derive(Deserialize, Debug)]
struct Config {
    general: General,
    modules: Modules,
    display: Display,
    art: Option<Art>,
}

#[derive(Deserialize, Debug)]
struct General {
    default_art: String,
    art_directory: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Display {
    textfield: DisplayTextField
}

#[derive(Deserialize, Debug)]
struct DisplayTextField {
    walls: Option<String>,
    gap: Option<usize>
}

#[derive(Deserialize, Debug)]
struct Modules {
    modules: Vec<String>,
    definitions: Vec<Module>,
}

#[derive(Deserialize, Debug)]
struct Module {
    name: String,
    key: Option<String>,
    format: Option<String>,
    walls: Option<bool>,
    #[serde(rename(deserialize = "type"))]
    module_type: String,
    execute: Option<Vec<String>>,
}

impl Config {
    fn get_config(info: &OsInfo, custom_configuration: (bool, Vec<String>)) -> Config {
        let config_dir = path::Path::new(dirs::config_dir().unwrap().as_path()).join(
            if info.os_type == "macos" {
                "se.spamix.fetch"
            } else {
                "fetch"
            },
        );
        let configuration_file = if custom_configuration.0 == true {
            custom_configuration.1.get(0).unwrap().to_owned()
        } else if config_dir.join("config.toml").try_exists().is_err() {
            "/etc/fetch/config.toml".to_string()
        } else {
            config_dir.join("config.toml").to_str().unwrap().to_string()
        };

        let default_configuration = r#"
    [general]
    art_directory = "erm"
    [modules]
    modules = []
    definitions = [{ name = "test", key = "tesstt", format = "{1} {2}", type = "command" }]"#;

        let file_content = match fs::read_to_string(configuration_file) {
            Ok(val) => val,
            Err(_) => {
                println!("[warning] Config file not found! Default configuration will be used");
                String::from(default_configuration)
            }
        };

        let config: Config = match toml::from_str(&file_content) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}", e);
                exit(1)
            }
        };

        config
    }
    fn parse_module(info: &OsInfo, module: Module) -> (String, (String, String)) {
        let name = &module.name;

        let os_release = info.os_release_file_content.os_release.clone();
         
        // format values
        let mut formats: Vec<String> = vec![];

        let mut value: String = String::new();

        let module_type = module.module_type.clone();

        // TODO: Add more modules
        match module_type.as_str() {
            "shell" => {formats.push(info.shell.clone())},
            "kernel" => {formats.push(info.os_release.clone())},
            "userhost" => {
                formats.push(info.username.clone());
                formats.push(info.hostname.clone());
            },
            "os" => {
                formats.push(match os_release.get("PRETTY_NAME") {
                    Some(val) => val.clone(),
                    None => info.os_type.clone(),
                });
                formats.push(match os_release.get("VERSION_ID") {
                    Some(val) => val.clone(),
                    None => "".to_string(),
                });
                formats.push(info.os_arch.clone());
            },
            "custom" => {
                value = match module.format.as_ref() {
                    Some(val) => {
                        val.clone()
                    },
                    None => {
                        if module.execute.is_none() {
                            eprintln!("Module \"custom\" may NOT have an empty format variable if variable execute isn't used!"); 
                            exit(1);
                        } else {
                            let execute_options = module.execute.unwrap().clone();
                            let execute_command_output = match Command::new(execute_options.get(0).unwrap()).args(execute_options[1..].iter()).output() {
                                Ok(val) => val.stdout,
                                Err(e) => {eprintln!("Error: Failed to execute: \"{}\"\nCommand Error: {}", execute_options.join(" "), e); exit(1);}
                            };
                            String::from_utf8_lossy(&execute_command_output).to_string()
                        }
                    }
                }
            },
            &_ => {
                eprintln!("Module {} is non-existant", module_type);
                exit(1);
            }
        };

        // Parse the format string, except if the module type is custom
        if module_type.as_str() != "custom" {
            match module.format {
                Some(_) => {
                    for part in module.format.as_ref().unwrap().split_inclusive('}') {
                        // find where the format part starts
                        // If this fails, just skip it since there's obviously no format part
                        let start_index = match part.find("{") {
                            Some(val) => val,
                            None => continue,
                        };
                        // Get the part that isnt a format
                        let rest_part = &part[..start_index];
                        // get what index of the module values the format requests
                        let index = match &part[start_index + 1..start_index + 2]
                            .to_string()
                            .parse::<usize>()
                        {
                            Ok(val) => val.clone() - 1,
                            Err(_) => {
                                eprintln!("Failed to get index format in string:\n{}", part);
                                exit(1);
                            }
                        };

                        if index >= formats.len() {
                            eprintln!("Error! Format index is bigger than the module returns\n>>>\"{}\"<<<", part);
                            exit(1);
                        }

                        // empty values are skipped
                        let output = if !formats[index].is_empty() {
                            format!("{}{}", rest_part, formats[index])
                        } else {
                            "".to_string()
                        };
                        value.push_str(&output);
                    }
                }
                None => {
                    formats.into_iter().for_each(|val| {
                        value.push_str(val.as_str());
                        if !val.is_empty() {
                            value.push(' ')
                        }
                    });
                }
            };
        }
        let key = match module.key {
            Some(val) => val,
            None => "".to_string()
        };
        (name.to_owned(), (key.to_string(), value))
    }
}

// This is where you add art if you want the path to be configurable
#[derive(Deserialize, Debug)]
struct Art {
    linux: Option<String>,
    macos: Option<String>,
    freebsd: Option<String>,
    netbsd: Option<String>,
    openbsd: Option<String>,
}

impl Art {
    fn get_art(&self, query: &String) -> Option<String> {
        let returnval: &Option<String>;
        // add the art and it's corresponding ID here!
        returnval = match query.to_lowercase().as_str() {
            "linux" => &self.linux,
            "macos" => &self.macos,
            "freebsd" => &self.freebsd,
            "netbsd" => &self.netbsd,
            "openbsd" => &self.openbsd,
            _ => &None,
        };
        returnval.to_owned()
    }
}

#[derive(Debug, Clone)]
struct OsRelease {
    // Old value I might bring back
    //exists: bool,
    os_release: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct OsInfo {
    os_release_file_content: OsRelease,
    os_type: String,
    os_arch: String,
    shell: String,
    username: String,
    os_release: String,
    hostname: String,
}

impl OsRelease {
    fn new() -> OsRelease {
        let mut os_release_values: HashMap<String, String> = HashMap::new();
        // check if the file exists
        let os_release_file_path = path::Path::new("/etc/os-release");
        let os_release_file_exists = match path::Path::try_exists(os_release_file_path) {
            Ok(_) => true,
            Err(_) => false,
        };

        // If file exists, read the contents
        if os_release_file_exists {
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
        };
        OsRelease {
            //exists: os_release_file_exists,
            os_release: os_release_values,
        }
    }
}

impl OsInfo {
    fn new() -> Self {
        let uname = utsname::uname().unwrap();
        return Self {
            os_release_file_content: OsRelease::new(),
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
            os_release: String::from(uname.release().to_str().unwrap()),
            hostname: String::from(uname.nodename().to_str().unwrap()),
        };
    }
}

fn get_ascii(info: &OsInfo, custom_logo: Option<String>, config: &Config) -> String {
    let os_type = if !custom_logo.is_none() {
        custom_logo.as_ref().unwrap().to_owned()
    } else {
        match info.os_release_file_content.os_release.get("ID") {
            Some(val) => val.clone(),
            None => info.os_type.clone(),
        }
    };

    let config_dir =
        path::Path::new(dirs::config_dir().unwrap().as_path()).join(if info.os_type == "macos" {
            "se.spamix.fetch"
        } else {
            "fetch"
        });

    let art_directory = match &config.general.art_directory {
        Some(val) => path::Path::new(val.as_str()).to_path_buf(),
        None => {
            if config_dir.join("art").exists() == true {
                config_dir.join("art")
            } else if path::Path::new("/etc/fetch/art/").exists() == true {
                path::Path::new("/etc/fetch/art").to_path_buf()
            } else {
                println!("Error: No art directory is present. Please create either \"/etc/fetch/art/\" or \"{}/art\" and install the required art!", config_dir.to_str().unwrap());
                exit(1);
            }
        }
    };

    let art_path = match path::Path::new(config.general.default_art.as_str()).exists() {
        true => path::Path::new(config.general.default_art.as_str()).to_path_buf(),
        false => art_directory.join("default"),
    };
    let mut art: String;
    if art_path.exists() && custom_logo.is_none() {
        art = fs::read_to_string(&art_path).unwrap();
        if art.is_empty() {
            eprintln!(
                "Error! {} is present but empty - {} may NOT be empty",
                art_path.to_str().unwrap(),
                art_path.to_str().unwrap()
            );
            exit(1);
        }
    } else if config.art.is_some() && config.art.as_ref().unwrap().get_art(&os_type).is_some() {
        art = config.art.as_ref().unwrap().get_art(&os_type).unwrap();
    } else {
        art = match fs::read_to_string(&art_directory.join("unknown")) {
            Ok(val) => val,
            Err(_) => {
                println!("No \"unknown\" art is present! Please install the necessary art.");
                exit(1);
            }
        };
        let paths = fs::read_dir(&art_directory).unwrap();
        // find the correct art file
        match paths
            .into_iter()
            .find(|path| path.as_ref().unwrap().file_name().to_str().unwrap() == &os_type)
        {
            Some(val) => art = fs::read_to_string(val.unwrap().path()).unwrap(),
            _ => (),
        }
    };
    art
}

fn create_output(art: String, info: OsInfo, modules: Modules, display: Display) -> String {
    // Preparation starts here

    // initialize the output string
    let mut outstr = String::new();

    // initialize temporary "fields strings"
    let mut tmp_fieldstrings: Vec<String> = vec![];

    // get all art lines
    let art_lines: Vec<&str> = art.split("\n").filter(|&x| !x.is_empty()).collect();

    // get all the fields
    let user_host = format!(" {}@{} ", info.username, info.hostname);
    let os_release_file = info.os_release_file_content.os_release.clone();
    let kernel = &info.os_release;
    let shell = &info.shell;

    // start of module section
    // rework THIS
    // Vector of (key, text)
    let mut parsed_modules: HashMap<String, (String, String)> = HashMap::new();
    for module in modules.definitions {
        let parsed = Config::parse_module(&info, module);
        parsed_modules.insert(parsed.0, (parsed.1.0, parsed.1.1));
    }
    // get longest module
    let longest_module = match parsed_modules
        .iter()
        .max_by(|x, y| x.0.len().cmp(&y.0.len()))
    {
        Some(val) => val.0.chars().count(),
        None => {
            eprintln!("Error: All modules are empty");
            exit(1);
        }
    };
    /*let module_names = [
        "",
        "OS",
        "Arch",
        if &info.os_type == "linux" {
            "Kernel"
        } else {
            "Release"
        },
        "Shell",
    ];*/

    modules.modules.iter().for_each(|val| {
        let module = match parsed_modules.get(val) {
            Some(v) => v,
            None => {eprintln!("Error! Module \"{}\" is undefined", val); exit(1);}
        };
        let numspaces = &longest_module - &module.0.len();
        tmp_fieldstrings.push(format!(
                "{}:{:>numspaces$}",
                module.0, module.1
            )); 
    });

    // Add all fields to the vector
    /*for i in 0..module_names.len() {
        if i == 0 {
            tmp_fieldstrings.push(format!("┏{:━>longest_module$}┓", ""));
        }
        // rework this
        if module_names[i] != "" {
            let numspaces = &longest_module - &module_names[i].len() - 3;
            tmp_fieldstrings.push(format!(
                "┃ {}:{:>numspaces$} ┃",
                module_names[i], modules[i]
            ));
        } else {
            tmp_fieldstrings.push(format!("┃{}┃", modules[i]));
            if i == 0 {
                tmp_fieldstrings.push(format!("┣{:━>longest_module$}┫", ""))
            }
        };
        if i == module_names.len() - 1 {
            tmp_fieldstrings.push(format!("┗{:━>longest_module$}┛", ""));
        }
    }*/

    // get how long the output will be in lines

    let out_length = if tmp_fieldstrings.len() > art_lines.len() {
        tmp_fieldstrings.len()
    } else {
        art_lines.len()
    };

    // get how long the field or the ascii art should wait
    // returns (true, usize) if the field is bigger than the ascii art
    // returns (false, usize) if the field is smaller than the ascii art
    let wait = if tmp_fieldstrings.len() > art_lines.len() {
        (
            true,
            ((tmp_fieldstrings.len() as f32 / 2.0).floor() - (art_lines.len() as f32 / 2.0).floor())
                as usize,
        )
    } else {
        (
            false,
            ((art_lines.len() as f32 / 2.0).floor() - (tmp_fieldstrings.len() as f32 / 2.0).floor())
                as usize,
        )
    };

    // get the longest art or field line
    let longest_art_line = if wait.0 != true {
        match tmp_fieldstrings.iter().max_by(|x, y| x.len().cmp(&y.len())) {
            Some(val) => val.chars().count(),
            None => {
                eprintln!("Error: Field strings are empty");
                exit(1);
            }
        }
    } else {
        match art_lines.iter().max_by(|x, y| x.len().cmp(&y.len())) {
            Some(val) => val.chars().count(),
            None => {
                eprintln!("Error: Art file is empty");
                exit(1);
            }
        }
    };

    // create counter
    let mut wait_counter = wait.1.clone();

    // Output creation starts from here

    // combine art and fields into one output
    for i in 0..out_length {
        let spaces_needed: usize;
        // create the correct lines and also get the correct amount of spaces
        //
        let line1;
        let line2;
        if wait.0 == true {
            line1 = if wait_counter == 0 && i - wait.1 < art_lines.len() {
                spaces_needed = longest_art_line - art_lines[i - wait.1].len();
                art_lines[i - wait.1].to_string()
            } else {
                spaces_needed = longest_art_line;
                String::from("")
            };
            line2 = if i < tmp_fieldstrings.len() {
                tmp_fieldstrings[i].as_str().to_string()
            } else {
                String::from("")
            };
        } else {
            line1 = if i < art_lines.len() {
                art_lines[i].to_string()
            } else {
                String::from("")
            };
            line2 = if wait_counter == 0 && i - wait.1 < tmp_fieldstrings.len() {
                spaces_needed = 0;
                tmp_fieldstrings[i - wait.1].to_string()
            } else {
                spaces_needed = longest_art_line;
                String::from("")
            };
        }
        if wait_counter != 0 {
            wait_counter -= 1;
        };
        // get the so called "2nd line", the line that isn't affected by wait
        // get either field or art
        outstr.push_str(format!("  {}{:>spaces_needed$}  {}\n", line1, "", line2).as_str());
    }

    outstr
}
