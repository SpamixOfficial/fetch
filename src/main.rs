use dirs;
use std::{collections::HashMap, fs, path, process::exit, str};

use taap;

pub mod config;
pub mod osrelease;

use config::{Config, Display, Modules};
use osrelease::OsInfo;

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
    let config = Config::get_config(&info, args.get("c").unwrap().to_owned());

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

    let output = create_output(art, info, config.modules, config.display, &debug);
    println!("{}", output);
}

fn get_ascii(info: &OsInfo, custom_logo: Option<String>, config: &Config) -> String {
    // Get the OS name also known as the os "ID"
    // We get the custom logo if specified, if that fails we get the os-release ID.
    // If that fails we get the os_type
    //
    // Later in the code if the logo for the os turn out to not be present we use the os_type instead!
    let os_type = custom_logo.unwrap_or(match info.os_release_file_content.os_release.get("ID") {
        Some(val) => val.to_owned(),
        None => info.os_type.clone(),
    });

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
        art = match fs::read_to_string(&art_directory.join("unknown")) {
            Ok(val) => val,
            Err(_) => {
                println!("No \"unknown\" art is present! Please install the necessary art.");
                exit(1);
            }
        };
        // find the correct art file
        // We first try to use our os_type variable, but if that isnt found we go directly to the
        // actual os_type, "linux", "freebsd", "macos", etc...
        //
        // If this fails we simply do nothing because we've already handled the "unknown" art before
        match fs::read_dir(&art_directory)
            .unwrap()
            .into_iter()
            .find(|path| {
                path.as_ref().unwrap().file_name().to_str().unwrap() == &os_type
                    || path.as_ref().unwrap().file_name().to_str().unwrap() == &info.os_type
            }) {
            Some(val) => art = fs::read_to_string(val.unwrap().path()).unwrap(),
            _ => (),
        }
    };
    art
}

fn create_output(
    art: String,
    info: OsInfo,
    modules: Modules,
    display: Display,
    debug: &bool,
) -> String {
    // Preparation starts here

    // initialize the output string
    let mut outstr = String::new();

    // initialize temporary "fields strings"
    let mut tmp_fieldstrings: Vec<String> = vec![];

    // get all art lines and add necessary spaces
    let mut art_lines: Vec<&str> = art.split("\n").filter(|&x| !x.is_empty()).collect();

    let mut tmp_art_lines: Vec<String> = vec![];

    // Find the longest "item" (string/line) in the art file/object
    // Also return the art lines as a Vector
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
    //
    // IMPORTANT: Dont confuse the longest module with the longest art line (longest_item)
    // The variables look the same but they are used for different purposes!
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

    if *debug {
        dbg!(&longest_module);
    }

    // This section gets some variables used for the textfield

    // Get the separator from config, default to ":"
    // Also get separator module
    //
    // Separator module != Separator setting
    //
    //   SOME TEXT
    //  +++++++++++ <-- That is the separator module in action
    //  Param: Text
    //       ^
    //       That is the separator setting

    let separator = display.textfield.separator.unwrap_or(":".to_string());
    let textfield_walls = match display.textfield.walls {
        Some(val) => val,
        None => "".to_owned(),
    };

    dbg!(&textfield_walls);

    // Create textfield output
    modules.modules.iter().for_each(|val| {
        let module = match parsed_modules.get(val) {
            Some(v) => {
                let mut v_clone = v.clone();
                if v.2 == "separator" {
                    v_clone.1 = String::new();
                    let sep_char = v.1.chars().collect::<Vec<char>>()[0];
                    for _ in 0..longest_module {
                        v_clone.1.push(sep_char)
                    }
                };
                v_clone
            }
            None => {
                eprintln!("Error! Module \"{}\" is undefined", val);
                exit(1);
            }
        };
        if *debug {
            dbg!(&module);
        };
        // get number of spaces
        // I remove 1 space on the default option because the padding becomes janky and adds one extra space
        // No idea what is the cause of this ¯\_(ツ)_/¯
        //
        // If you happen to find the reason/fix for this, please do a better implementation :-)
        let numspaces = match display.textfield.gap {
            Some(val) => val + module.1.len(),
            None => &longest_module - module.0.len() - 1,
        };
        dbg!(numspaces, module.0.len(), separator.len(),);
        tmp_fieldstrings.push(format!(
            "{}{}{}{:>spaces$}{}",
            textfield_walls,
            module.0,
            if !module.0.is_empty() { &separator } else { "" },
            module.1,
            textfield_walls,
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
        outstr.push_str(
            format!(
                "  {}{:>spaces_needed$}{:>displaygap$}  {}\n",
                line1,
                "",
                "",
                line2,
                displaygap = if display.gap.is_some() {
                    display.gap.unwrap()
                } else {
                    0
                }
            )
            .as_str(),
        );
    }

    outstr
}
