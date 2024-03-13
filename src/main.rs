use dirs;
use std::{
    collections::HashMap,
    fs,
    path,
    process::exit,
    str,
};

use taap;

pub mod config;
pub mod osrelease;

use osrelease::OsInfo;
use config::{Config, Modules, Display};

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
    let args = arguments.parse_args(None);
    
    dbg!(&args.get("c"));

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
                if !cfg!(target_os = "macos") {
                    println!("Error: No art directory is present. Please create either \"/etc/fetch/art/\" or \"{}/art\" and install the required art!", config_dir.to_str().unwrap());
                } else {
                    println!("Error: No art directory is present. Please create \"{}/art\" and install the required art!", config_dir.to_str().unwrap());
                }
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

    // get all art lines and add necessary spaces
    let mut art_lines: Vec<&str> = art.split("\n").filter(|&x| !x.is_empty()).collect();

    let mut tmp_art_lines: Vec<String> = vec![];

    art_lines = match art_lines.iter().max_by(|x, y| x.len().cmp(&y.len())) {
        Some(val) => {
            let longest_item = val.chars().count();
            art_lines
                .iter()
                .for_each(|x| tmp_art_lines.push(format!("{:<longest_item$}", x)));
            tmp_art_lines.iter().map(|s| s.as_str()).collect::<Vec<_>>()
        }
        None => {
            eprintln!("Error: Art file is empty");
            exit(1);
        }
    };

    // start of module section

    let mut parsed_modules: HashMap<String, (String, String, String)> = HashMap::new();
    for module in modules.definitions {
        let parsed = Config::parse_module(&info, module);
        parsed_modules.insert(parsed.0, (parsed.1 .0, parsed.1 .1, parsed.1 .2));
    }
    
    // get longest module
    let longest_module = match parsed_modules
        .iter()
        .max_by(|x, y| (x.1 .0.len() + x.1 .1.len()).cmp(&(y.1 .0.len() + y.1 .1.len())))
    {
        Some(val) => val.1 .0.len() + val.1 .1.len(),
        None => {
            eprintln!("Error: All modules are empty");
            exit(1);
        }
    };
    
    dbg!(&longest_module);

    let separator = display.textfield.separator.unwrap_or(":".to_string());
    modules.modules.iter().for_each(|val| {
        let module = match parsed_modules.get(val) {
            Some(v) => {
                let mut v_clone = v.clone();
                if v.2 == "separator" {
                    // TODO: FINISH SEPARATOR MODULE HERE (This creates the actual line)
                    //v_clone.1 = format!("{:>length$}", vstr = v.1, length = longest_module)
                    v_clone.1 = String::new();
                    let sep_char = v.1.chars().collect::<Vec<char>>()[0];
                    for _ in 0..longest_module {
                        v_clone.1.push(sep_char)
                    };
                };
                v_clone
            }
            None => {
                eprintln!("Error! Module \"{}\" is undefined", val);
                exit(1);
            }
        };
        dbg!(&module);
        // get number of spaces
        let numspaces = match display.textfield.gap {
            Some(val) => val - module.0.len() - separator.len(),
            None => &longest_module - module.0.len(),
        };
        tmp_fieldstrings.push(format!(
            "{}{}{:>spaces$}",
            module.0,
            if !module.0.is_empty() { &separator } else { "" },
            module.1,
            spaces = if !module.0.is_empty() { numspaces } else { 0 }
        ));
    });

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
