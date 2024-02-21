use nix::sys::utsname;

use std::{collections::HashMap, env, fs, io::Read, path, process::exit, str};
use taap;

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
                None => String::from("unknown"),
            },
            os_release: String::from(uname.release().to_str().unwrap()),
            hostname: String::from(uname.nodename().to_str().unwrap()),
        }
    }
}

fn get_ascii(info: &OsInfo, custom_logo: Option<String>) -> String {
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
    let art_path = path::Path::new("/etc/ascii-art");
    let art: String;
    if path::Path::exists(art_path) {
        art = fs::read_to_string("/etc/ascii-art").unwrap();
        if art.is_empty() {
            eprintln!(
                "Error! /etc/ascii-art is present but empty - /etc/ascii-art may NOT be empty"
            );
            exit(1);
        }
    } else {
        art = if os_type == "linux" {
            std_linux_art.to_string()
        } else if os_type == "freebsd" {
            std_freebsd_art.to_string()
        } else if os_type == "openbsd" {
            std_openbsd_art.to_string()
        } else if os_type == "netbsd" {
            std_netbsd_art.to_string()
        } else if os_type == "macos" {
            std_macos_art.to_string()
        } else {
            std_unknown_art.to_string()
        };
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

fn main() {
    let mut arguments = taap::Argument::new(
        "fetch",
        "Minimal and easy fetch tool written in rust",
        "",
        "SpamixOfficial 2024",
    );
    arguments.add_option('-', "os-logo", "1", Some("Manually specify OS logo"));
    arguments.add_exit_status(0, "Everything went well");
    arguments.add_exit_status(1, "An error occurred");
    let args = arguments.parse_args();
    let os_logo = args.get("os-logo").unwrap();
    let info = OsInfo::new();
    let art;
    if os_logo.0 {
        art = get_ascii(&info, Some(os_logo.1.get(0).unwrap().to_owned()));
    } else {
        art = get_ascii(&info, None);
    };
    let output = create_output(art, info);
    println!("{}", output);
}
