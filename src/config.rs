use crate::osrelease::OsInfo;
use dirs;
use serde::Deserialize;
use std::{
    fs, path,
    process::{exit, Command},
    str,
};

use toml;


// Art section
// This is where you add art if you want the path to be configurable
#[derive(Deserialize, Debug)]
pub struct Art {
    pub linux: Option<String>,
    pub macos: Option<String>,
    pub freebsd: Option<String>,
    pub netbsd: Option<String>,
    pub openbsd: Option<String>,
}

impl Art {
    pub fn get_art(&self, query: &String) -> Option<String> {
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



// Config section

#[derive(Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub modules: Modules,
    pub display: Display,
    pub art: Option<Art>,
}

#[derive(Deserialize, Debug)]
pub struct General {
    pub default_art: String,
    pub art_directory: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Display {
    pub textfield: DisplayTextField,
    pub gap: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct DisplayTextField {
    pub separator: Option<String>,
    pub walls: Option<String>,
    pub gap: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct Modules {
    pub modules: Vec<String>,
    pub definitions: Vec<Module>,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub key: Option<String>,
    pub format: Option<String>,
    pub separator_char: Option<char>,
    pub walls: Option<bool>,
    #[serde(rename(deserialize = "type"))]
    pub module_type: String,
    pub execute: Option<Vec<String>>,
}

impl Config {
    pub fn get_config(info: &OsInfo, custom_configuration: (bool, Vec<String>)) -> Config {
        let config_dir = path::Path::new(dirs::config_dir().unwrap().as_path()).join(
            if info.os_type == "macos" {
                "se.spamix.fetch"
            } else {
                "fetch"
            },
        );
        dbg!(&custom_configuration);
        let configuration_file = if custom_configuration.0 == true {
            custom_configuration.1.get(0).unwrap().to_owned()
        } else if config_dir.join("config.toml").try_exists().is_err() {
            "/etc/fetch/config.toml".to_string()
        } else {
            config_dir.join("config.toml").to_str().unwrap().to_string()
        };

        let default_configuration = r#"
[general]
default_art = "~/.config/fetch/art/default"

[display]
[display.textfield]
[modules]
modules = ["userhost", "separator", "shell", "os", "kernel"]

definitions = [{name = "separator", separator_char = '-', type = "separator"},{name = "kernel", key = "KERNEL", type = "kernel"}, {name = "shell", key = "SHELL", type = "shell"},{name = "userhost", format = "{1}@{2}", type = "userhost"},{name = "os", key = "OS", type = "os"}]"#;

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
    pub fn parse_module(info: &OsInfo, module: Module) -> (String, (String, String, String)) {
        let name = &module.name;

        let os_release = info.os_release_file_content.os_release.clone();

        // format values
        let mut formats: Vec<String> = vec![];

        let mut value: String = String::new();

        let module_type = module.module_type.clone();

        // TODO: Add more modules
        match module_type.as_str() {
            "shell" => formats.push(info.shell.clone()),
            "kernel" => formats.push(info.os_release.clone()),
            "userhost" => {
                formats.push(info.username.clone());
                formats.push(info.hostname.clone());
            }
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
            }
            "separator" => formats.push(match module.separator_char {
                Some(val) => val.to_string().clone(),
                None => {
                    eprintln!(
                        "The \"separator\" needs the \"separator_char\" parameter to be specified"
                    );
                    exit(1);
                }
            }),
            "custom" => {
                value = match module.format.as_ref() {
                    Some(val) => val.clone(),
                    None => {
                        if !module.execute.is_none() {
                            let execute_options = module.execute.unwrap().clone();
                            let execute_command_output =
                                match Command::new(execute_options.get(0).unwrap())
                                    .args(execute_options[1..].iter())
                                    .output()
                                {
                                    Ok(val) => val.stdout,
                                    Err(e) => {
                                        eprintln!(
                                            "Error: Failed to execute: \"{}\"\nCommand Error: {}",
                                            execute_options.join(" "),
                                            e
                                        );
                                        exit(1);
                                    }
                                };
                            String::from_utf8_lossy(&execute_command_output).to_string()
                        } else {
                            eprintln!("Module \"custom\" may NOT have an empty format variable if variable execute isn't used!");
                            exit(1);
                        }
                    }
                }
            }
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
                    formats.iter().enumerate().for_each(|val| {
                        value.push_str(val.1.as_str());
                        dbg!(&val);
                        if !val.1.is_empty() && val.0 != formats.len() - 1 {
                            value.push(' ')
                        }
                    });
                }
            };
        }
        let key = match module.key {
            Some(val) => val,
            None => "".to_string(),
        };
        (name.to_owned(), (key.to_string(), value, module_type))
    }
}
