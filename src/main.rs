use dirs;
use std::{fs, path, process::exit};

use taap;

pub mod config;
pub mod osrelease;

use config::{Config, Display, ModuleType, Modules};
use osrelease::OsInfo;

use crate::config::ParsedModuleObject;

fn main() {
    // Argument creation and handling

    let debug: bool;

    let mut arguments = taap::Argument::new(
        "fetch",
        "Minimal and easy fetch tool written in rust",
        "",
        "SpamixOfficial 2024",
    );
    arguments.add_option('c', "config", "1", Some("Manually specify the config file"));
    arguments.add_option('-', "os-logo", "1", Some("Manually specify OS logo"));
    arguments.add_option('d', "debug", "0", None);
    // todo
    //arguments.add_option('l', "list-art", "0", Some("List all known available art"));
    arguments.add_exit_status(0, "Everything went well");
    arguments.add_exit_status(1, "An error occurred");
    let args = arguments.parse_args(None);
    debug = args.get("d").unwrap().0;

    if debug {
        dbg!(&args.get("c"));
    };

    // Start of program
    let info = OsInfo::new();
    let config = Config::get_config(&info, args.get("c").unwrap().to_owned(), debug);

    if debug {
        dbg!(&config);
    };

    let os_logo = args.get("os-logo").unwrap();
    let art;
    // If os-logo argument was used, use the specified logo
    // If you find these comments while looking through the code, I am so sorry you have to read
    // this spaghetti mess.
    //
    // I am unsure about what some of these parts do - I wrote them 6 months before this comment
    // ¯\_(ツ)_/¯
    if os_logo.0 {
        art = get_ascii(&info, Some(os_logo.1.get(0).unwrap().to_owned()), &config);
    } else {
        art = get_ascii(&info, None, &config);
    };

    let output = create_output(art, info, config.modules, config.display, debug);
    println!("{}", output);
}

fn get_ascii(info: &OsInfo, custom_logo: Option<String>, config: &Config) -> String {
    // Get the OS name also known as the os "ID"
    // We get the custom logo if specified, if that fails we get the os-release ID (distro id).
    // If that fails we get the os_type
    //
    // TLDR: custom logo > distro name > os name
    //
    // Later in the code if the logo for the os turns out to not be present we use this instead!
    let art_name = custom_logo.unwrap_or(
        info.os_release_file_content
            .os_release
            .get("ID")
            .unwrap_or(&info.os_type)
            .to_string(),
    );

    // Really weird way of getting the configuration directory, but it works
    //
    // Ignore the weird rust formatting
    let config_dir =
        path::Path::new(dirs::config_dir().unwrap().as_path()).join(if info.os_type == "macos" {
            "se.spamix.fetch"
        } else {
            "fetch"
        });

    // As you see we can define a custom art_directory in the config, which we try to use if it
    // exists
    //
    // If it doesnt exist we automaticaly determine it by doing witchcraft and spitting errors
    let art_directory = match &config.general.art_directory {
        Some(val) => path::Path::new(val.as_str()).to_path_buf(),
        None => {
            if config_dir.join("art").exists() {
                config_dir.join("art")
            } else if path::Path::new("/etc/fetch/art/").exists() {
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

    // Here we get the actual art path by either using the default art defined in the config
    // or by just using the file named "default" in the art directory
    //
    // If the default_art happens to not be defined, simply set the path to "" which will fail 100%
    let art_path = if config.general.default_art.is_some() {
        match path::Path::new(config.general.default_art.clone().unwrap().as_str()).exists() {
            true => {
                path::Path::new(config.general.default_art.clone().unwrap().as_str()).to_path_buf()
            }
            false => art_directory.join("default"),
        }
    } else {
        path::Path::new("").to_path_buf()
    };
    let mut art: String;
    if art_path.exists() {
        art = fs::read_to_string(&art_path).unwrap();
        if art.is_empty() {
            eprintln!(
                "Error! {} is present but empty - {} may NOT be empty",
                art_path.to_str().unwrap(),
                art_path.to_str().unwrap()
            );
            exit(1);
        }
    /*} else if config.art.is_some() && config.art.as_ref().unwrap().get_art(&os_type).is_some() {
    art = config.art.as_ref().unwrap().get_art(&os_type).unwrap();*/
    } else {
        // Here we set the art to unknown in case no art corresponding to your OS is actually
        // found!
        //
        // If this fails we can assume that the binary wasnt installed in a correct way
        // The outcome is us complaining about no art being installed
        // ¯\_(ツ)_/¯
        art = fs::read_to_string(&art_directory.join("unknown"))
            .expect("No \"unknown\" art is present! Please install the necessary art.");
        // find the correct art file
        // We first try to use our os_type variable, but if that isnt found we go directly to the
        // actual os_type, "linux", "freebsd", "macos", etc...
        //
        // If this fails we simply do nothing because we've already handled the "unknown" art before
        if let Some(file_name) = fs::read_dir(&art_directory)
            .unwrap()
            .into_iter()
            .filter_map(|p| p.ok())
            .map(|p| p.file_name().to_string_lossy().to_string())
            .find(|file_name| *file_name == art_name || *file_name == info.os_type)
        {
            art = fs::read_to_string(art_directory.join(file_name)).unwrap();
        }
    };
    art
}

fn create_output(
    art: String,
    info: OsInfo,
    modules: Modules,
    display: Display,
    debug: bool,
) -> String {
    // Preparation starts here

    // initialize output lines
    let mut lines = Vec::new();

    // initialize temporary "fields strings"
    let mut tmp_fieldstrings: Vec<String> = Vec::new();

    // get all art lines and add necessary spaces
    let mut art_lines: Vec<String> = art
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    // Find the longest "item" (string/line) in the art file/object
    let longest_item = art_lines
        .iter()
        .max_by_key(|x| x.len())
        .expect("Error: Art file is empty")
        .len();
    art_lines = art_lines
        .iter()
        .map(|x| format!("{:<longest_item$}", x))
        .collect();

    // start of module section

    let mut parsed_modules: Vec<ParsedModuleObject> = vec![];
    for module in modules.definitions {
        let parsed = Config::parse_module(&info, module, debug);
        parsed_modules.push(parsed);
    }

    if debug {
        dbg!(&parsed_modules);
    }

    // get longest module
    //
    // IMPORTANT: Dont confuse the longest module with the longest art line (longest_item)
    // The variables look the same but they are used for different purposes!
    let longest_module = parsed_modules
        .iter()
        .map(|m| m.key.len() + m.parsed_module.len())
        .max()
        .expect("Error: All modules are empty");

    if debug {
        dbg!(&longest_module);
    }

    // This section gets some variables used for the textfield

    // Get the separator from config, default to ": "
    // Also get separator module
    //
    // Separator module != Separator setting
    //
    //   SOME TEXT
    //  +++++++++++ <-- That is the separator module in action
    //  Param: Text
    //       ^
    //       That is the separator setting

    let separator = display.textfield.separator.unwrap_or(": ".to_string());
    let textfield_walls = display.textfield.walls.unwrap_or(String::from(""));

    if debug {
        dbg!(&textfield_walls);
    }

    // Create textfield output loop
    modules.modules.iter().for_each(|val| {
        // Start of the most spaghetti-code-like part
        // Will explain every part of this mess right here
        //
        // Here we find the right struct, and put it in an option so that we can correctly use it
        // Why the option you might ask, well we need to check if the module definition actually
        // exists
        let mut found_struct: Option<ParsedModuleObject> = None;
        parsed_modules.iter().for_each(|m| {
            if m.name == *val {
                found_struct = Some(m.clone())
            }
        });
        // In this statement we test if the module is defined or not.
        // If it isnt, we throw an error which basically begs the user to fix their configuration
        let module = match found_struct {
            Some(v) => {
                // In case we find it, we clone the "v" variable because that is the easiest way to
                // not fuck up borrows
                let mut v_clone = v.clone();
                // In case this is the fancy separator module, we have a special case for it
                // If not, we simply return the v_clone val!
                if v.module_type == ModuleType::Separator {
                    // Here we set v_clone.1 to a new string, because we will override the string
                    // we got with a new output. The output in v_clone.1 is just the raw-char from
                    // the beginning, therefore we need to change it
                    v_clone.parsed_module = String::new();
                    // Here we check if a format was used by literally checking if a format string
                    // exists
                    //
                    // We do this is because formats is tricky with this dynamic separator module
                    match v.format_string.clone() {
                        None => {
                            // If there isnt a format string we just do the usual stuff
                            // That is looping for a length and inserting it
                            let sep_char: char = v.parsed_module.chars().collect::<Vec<char>>()[0];
                            // Cursed math statement but turned out to be the easiest way possible
                            for _ in 0..longest_module + (if !v.walls { 2 } else { 0 }) {
                                v_clone.parsed_module.push(sep_char)
                            }
                        }
                        Some(_) => {
                            // Get the parts, that is before and after the format val (the
                            // separator)
                            let val_parts: Vec<String> = v
                                .parsed_module
                                .clone()
                                .split("<!")
                                .map(|f| f.to_string())
                                .collect::<Vec<String>>();
                            if val_parts.len() != 3 {
                                eprintln!("Separator module only allows 1 format statement");
                                exit(1);
                            }
                            let sep_before = val_parts.get(0).unwrap();
                            let sep_after = val_parts.get(2).unwrap();
                            let sep = val_parts.get(1).unwrap();

                            v_clone.parsed_module.push_str(sep_before);

                            for _ in 0..(longest_module + (if !v.walls { 2 } else { 0 })
                                - sep_before.chars().count()
                                - sep_after.chars().count())
                            {
                                v_clone.parsed_module.push_str(sep)
                            }
                            v_clone.parsed_module.push_str(sep_after);
                        }
                    };
                }
                v_clone
            }
            None => {
                eprintln!("Error! Module \"{}\" is undefined", val);
                exit(1);
            }
        };
        if debug {
            dbg!(&module);
        };

        // get number of spaces
        let numspaces = match display.textfield.gap {
            Some(val) => val + module.parsed_module.len(),
            None => &longest_module - module.key.len() - separator.len(),
        };

        if debug {
            dbg!(&numspaces);
        }

        // Create all the fieldstrings
        // Disable walls if they happen to be disabled in the module
        tmp_fieldstrings.push(format!(
            "{}{}{}{:>spaces$}{}",
            if module.walls {
                textfield_walls.clone()
            } else { 
                String::from("")
            },
            module.key,
            if !module.key.is_empty() {
                &separator
            } else {
                ""
            },
            module.parsed_module,
            if module.walls {
                textfield_walls.clone()
            } else {
                String::from("")
            },
            spaces = if !module.key.is_empty() { numspaces } else { 0 }
        ));
    });

    // Start of part where we create the art for the output

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
    let longest_art_line = if wait.0 {
        &art_lines
    } else {
        &tmp_fieldstrings
    }
    .iter()
    .max_by_key(|x| x.len())
    .expect(if wait.0 {
        "Error: Art file is empty"
    } else {
        "Error: Field strings are empty"
    })
    .len();

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
        if wait.0 {
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
        lines.push(format!(
            "  {}{:>spaces_needed$}{:>displaygap$}  {}",
            line1,
            "",
            "",
            line2,
            displaygap = if display.gap.is_some() {
                display.gap.unwrap()
            } else {
                0
            }
        ));
    }

    lines.join("\n")
}
