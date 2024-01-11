use dirs;
use json;
use nix::sys::utsname;
use serde::Deserialize;
use std::{collections::HashMap, env, fs, io::Read, path, process::exit, str};
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
    arguments.add_exit_status(0, "Everything went well");
    arguments.add_exit_status(1, "An error occurred");
    let args = arguments.parse_args();

    // Start of program
    let info = OsInfo::new();
    let config = get_config(&info, args.get("c").unwrap().to_owned());

    let os_logo = args.get("os-logo").unwrap();
    let art;
    if os_logo.0 {
        art = get_ascii(&info, Some(os_logo.1.get(0).unwrap().to_owned()), &config);
    } else {
        art = get_ascii(&info, None, &config);
    };
    let output = create_output(art, info);
    println!("{}", output);
}

#[derive(Deserialize, Debug)]
struct Config {
    general: General,
    modules: Modules,
}

#[derive(Deserialize, Debug)]
struct General {
    default_art: String,
    art_config: String,
    art_directory: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Modules {
    modules: Vec<String>,
    definitions: Vec<Module>,
}

#[derive(Deserialize, Debug)]
struct Module {
    name: String,
    key: String,
    format: String,
    #[serde(rename(deserialize = "type"))]
    module_type: String,
    text: Option<String>,
    execute: Option<String>,
}

struct OsRelease {
    // Old value I might bring back
    //exists: bool,
    os_release: HashMap<String, String>,
}

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

fn get_config(info: &OsInfo, custom_configuration: (bool, Vec<String>)) -> Config {
    let config_dir =
        path::Path::new(dirs::config_dir().unwrap().as_path()).join(if info.os_type == "macos" {
            "se.spamix.fetch"
        } else {
            "fetch"
        });
    let configuration_file = if custom_configuration.0 == true {
        custom_configuration.1.get(0).unwrap().to_owned()
    } else if config_dir.join("config.toml").try_exists().is_err() {
        "/etc/fetch/config.toml".to_string()
    } else {
        config_dir.join("config.toml").to_str().unwrap().to_string()
    };

    dbg!(&configuration_file);

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

fn get_ascii(info: &OsInfo, custom_logo: Option<String>, config: &Config) -> String {
    let std_linux_art = "
  .~.
  /V\\
 // \\\\
/(   )\\
 ^`~'^
";
    let std_freebsd_art = "
  ...      ....      ...  
 .:;&x;.$&&&&&$&$$.+$&;:. 
 .;:;$&$&&$&$&$$;&&$x;:x. 
  .x&$$&$$$$$$$$$$x;;:X.  
  .$$$&$Xx+;;;;;;;;:;;X.  
 .&+X&X;;;;;;;;;:::::;;x. 
 .&;;++;;;;;;;;:::;;:::$. 
 .&;;;;;;;;;:::;;;;:::$$. 
  .+;;;;;;;::;;;;;;:;X$.  
  ..:;;;;;;;;;;;;;++;X.   
    .$:;;;;;;;;++++XX.    
      .:;;;;;;;;XX;.      
         .......          
";
    let std_openbsd_art = "


             @  @@ @@              
            @@%@@%%+#@%@ @@        
         @@@%#---=--:=++**@@       
       @#@*+--:+-::+::-=--*%@@@    
     @@@@#--=-:-:--:--:=::--+@@    
      @*:--:=-:-:-:::::-=-+*-@@    
     %%+=-+::-.::...:::::#=+**#@   
@%+*%@@+-+--:...:....::::==-+#*%   
 @*--+*-=:-:.:.:....:::::::+=::*@  
   #-::-:==:+.=---:.::::::-#-+==#% 
   @#-*#==:=-=-:=-.::--:::-:-.-#*+@
    @#@@#:-=:==*:-:::-:-=-=-=##@   
        *#*=:=-:--::-:-=::==#%@    
       @@@%@+-+-==:=+--+-+*%#@     
            #@%#*#**+***#@@@       
            @ @ %@@@*@@@@@         
                @   @              
";
    let std_netbsd_art = "
                  ++++++  
      +      ++++++       
     * ++++++++++++++     
      * +++++++++         
       *++++++            
       *                  
 ++ +     + ** * *   ** * 
  +++ + +++ **** **  ** **
   ++ +  ++ ** *   * ** **
 +  + ++  + **** * * **** 
           *              
            *             
            **            
             **           
";
    let std_macos_art = "                               
          =    
        ===    
    =   =  =   
 ==============
-------------  
=============  
************** 
 **************
  ++++++++++++ 
    ++    ++   
";
    let std_unknown_art = "
 #######  
##     ## 
      ##  
    ###   
   ##     
          
   ##     
";
    let os_type = if custom_logo != None {
        custom_logo.unwrap()
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

    let raw_art_index_content = match fs::read_to_string(config_dir.join("art").join("index.json"))
    {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1)
        }
    };

    let art_index = match json::parse(raw_art_index_content.as_str()) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1)
        }
    };

    dbg!(art_index);

    let art_directory = match &config.general.art_directory {
        Some(val) => val.to_owned(),
        None => {
            if config_dir.join("art").exists() == true {
                config_dir.join("art").to_str().unwrap().to_string()
            } else if path::Path::new("/etc/fetch/art/").exists() == true {
                "/etc/fetch/art/".to_string()
            } else {
                println!("Error: No art directory is present. Please create either \"/etc/fetch/art/\" or \"{}/art\" and install the required art!", config_dir.to_str().unwrap());
                exit(1);
            }
        }
    };

    dbg!(&art_directory);

    let art_path = path::Path::new(config.general.default_art.as_str());
    let art: String;
    if path::Path::exists(art_path) {
        art = fs::read_to_string(art_path).unwrap();
        if art.is_empty() {
            eprintln!(
                "Error! {} is present but empty - {} may NOT be empty",
                art_path.to_str().unwrap(),
                art_path.to_str().unwrap()
            );
            exit(1);
        }
    } else {
        art = "".to_string();
        let paths = fs::read_dir(art_directory).unwrap();
    };
    art
}

fn create_output(art: String, info: OsInfo) -> String {
    // Preparation starts here

    // initialize the output string
    let mut outstr = String::new();

    // initialize temporary "fields strings"
    let mut tmp_fieldstrings: Vec<String> = vec![];

    // get all art lines
    let art_lines: Vec<&str> = art.split("\n").filter(|&x| !x.is_empty()).collect();

    // get all the fields
    let user_host = format!(" {}@{} ", info.username, info.hostname);
    let os_release_file = info.os_release_file_content.os_release;
    let os = match os_release_file.get("PRETTY_NAME") {
        Some(val) => {
            /*if os_release_file.contains_key("ID_LIKE") {
                format!("{}({}-like)", val, os_release_file.get("ID_LIKE").unwrap())
            } else {
                val.clone()
            }*/
            val.clone()
        }
        None => info.os_type.clone(),
    };
    let arch = &info.os_arch;
    let kernel = &info.os_release;
    let shell = &info.shell;

    let params = [&user_host, &os, &arch, &kernel, &shell];
    // get longest param
    let longest_param = params
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .chars()
        .count();
    let param_names = [
        "",
        "OS",
        "Arch",
        if &info.os_type == "linux" {
            "Kernel"
        } else {
            "Release"
        },
        "Shell",
    ];

    // Add all fields to the vector
    for i in 0..param_names.len() {
        if i == 0 {
            tmp_fieldstrings.push(format!("┏{:━>longest_param$}┓", ""));
        }
        if param_names[i] != "" {
            let numspaces = &longest_param - &param_names[i].len() - 3;
            tmp_fieldstrings.push(format!("┃ {}:{:>numspaces$} ┃", param_names[i], params[i]));
        } else {
            tmp_fieldstrings.push(format!("┃{}┃", params[i]));
            if i == 0 {
                tmp_fieldstrings.push(format!("┣{:━>longest_param$}┫", ""))
            }
        };
        if i == param_names.len() - 1 {
            tmp_fieldstrings.push(format!("┗{:━>longest_param$}┛", ""));
        }
    }

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
        tmp_fieldstrings
            .iter()
            .max_by(|x, y| x.len().cmp(&y.len()))
            .unwrap()
            .chars()
            .count()
    } else {
        art_lines
            .iter()
            .max_by(|x, y| x.len().cmp(&y.len()))
            .unwrap()
            .chars()
            .count()
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
