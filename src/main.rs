use std::{collections::HashMap, env, fs, io::Read, path, str};
use taap;
use uname::uname;

struct OsRelease {
    exists: bool,
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

            // Split file content into lines
            let raw_file_lines: Vec<String> = raw_file_content
                .lines()
                .map(|value| value.to_string())
                .collect();
            raw_file_lines.iter().for_each(|line| {
                let values: Vec<&str> = line.splitn(2, "=").collect();
                os_release_values.insert(values.get(0).unwrap().to_string(), values.get(1).unwrap().to_string());
            });
        };
        dbg!(&os_release_values);
        OsRelease {
            exists: os_release_file_exists,
            os_release: os_release_values,
        }
    }
}

impl OsInfo {
    fn new() -> Self {
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
            os_release: uname().unwrap().release,
            hostname: uname().unwrap().nodename,
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
        info.os_type.clone()
    };
    let art_path = if &info.os_type == "linux" && &info.os_type == "freebsd" {
        path::Path::new("/etc/ascii-art")
    } else {
        path::Path::new("/etc/ascii-art")
    };
    let art: String;
    if path::Path::exists(art_path) {
        art = fs::read_to_string("/etc/ascii-art").unwrap();
    } else {
        art = if os_type == "linux" {
            std_linux_art.to_string()
        } else if os_type == "freebsd" {
            std_freebsd_art.to_string()
        } else {
            std_unknown_art.to_string()
        };
    };
    // debug
    // art = std_freebsd_art.to_string();
    art
}

fn create_output(art: String, info: OsInfo) -> String {
    // initialize the output string
    let mut outstr = String::new();

    // initialize temporary "fields strings"
    let mut tmp_fieldstrings: Vec<String> = vec![];

    // get all art lines
    let art_lines: Vec<&str> = art.split("\n").filter(|&x| !x.is_empty()).collect();

    // get all the fields
    let user_host = format!("{}@{}", info.username, info.hostname);
    let os = &info.os_type;
    let arch = &info.os_arch;
    let kernel = &info.os_release;
    let shell = &info.shell;

    let params = [&user_host, &os, &arch, &kernel, &shell];
    // get longest param (will redo)
    let mut lastlength = 0;
    for param in params {
        let length = param.chars().count();
        if length > lastlength {
            lastlength = length;
        };
    }

    // Add padding to the lastlength variable
    lastlength += 2;

    let param_names = ["", "OS", "Arch", "Kernel", "Shell"];
    // Add all fields to the vector
    for i in 0..param_names.len() {
        if i == 0 {
            tmp_fieldstrings.push(format!("┏{:━>lastlength$}┓", ""));
        }
        if param_names[i] != "" {
            let numspaces = &lastlength - &param_names[i].len() - 3;
            tmp_fieldstrings.push(format!("┃ {}:{:>numspaces$} ┃", param_names[i], params[i]));
        } else {
            tmp_fieldstrings.push(format!("┃ {} ┃", params[i]));
            if i == 0 {
                tmp_fieldstrings.push(format!("┣{:━>lastlength$}┫", ""))
            }
        };
        if i == param_names.len() - 1 {
            tmp_fieldstrings.push(format!("┗{:━>lastlength$}┛", ""));
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
