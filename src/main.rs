use std::{collections::HashMap, env, fs, path, process::exit, str};
use uname::uname;

struct Argument {
    args: HashMap<char, (String, usize)>,
}

impl Argument {
    fn new() -> Self {
        let args: HashMap<char, (String, usize)> = HashMap::new();
        Self { args }
    }
    fn add_arg(&mut self, mut short: char, long: &str, parameters: usize) {
        if short == ' ' {
            short = '-'
        };
        self.args.insert(short, (long.to_string(), parameters));
    }

    fn parse_args(&mut self) -> HashMap<String, (bool, Vec<String>)> {
        let raw_args = std::env::args();
        let mut collected_raw_args: Vec<String> = std::env::args().collect();
        collected_raw_args.remove(0);
        let arguments = &self.args;
        let mut return_map: HashMap<String, (bool, Vec<String>)> = HashMap::new();
        for key in arguments.keys() {
            return_map.insert(key.to_string(), (false, vec![]));
        }
        dbg!(&return_map);
        for (pos, argument) in raw_args.into_iter().enumerate() {
            if pos == 0 {
                continue;
            };
            dbg!(&argument);
            if argument.starts_with("-") && argument.chars().nth(1).unwrap() != '-' {
                for part in argument.get(1..).unwrap().chars() {
                    dbg!(&arguments.get(&part).unwrap().1);
                    dbg!(pos);
                    if arguments.contains_key(&part) {
                        let arguments_needed = arguments.get(&part).unwrap().1;
                        dbg!(collected_raw_args.len());
                        if collected_raw_args.len() < pos + arguments_needed {
                            eprintln!("Too few arguments passed to -{}", &part);
                            exit(1);
                        }
                        *return_map.get_mut(&part.to_string()).unwrap() = (
                            true,
                            collected_raw_args[pos..(pos + arguments_needed)]
                                .iter()
                                .cloned()
                                .collect(),
                        );
                    };
                }
            } else if argument.len() > 2 && argument.get(..2).unwrap() == "--" {
                let part = argument.get(2..).unwrap();
                dbg!(part);
                for field in &*arguments {
                    let key = field.0;
                    let values = field.1;
                    if part == values.0 {
                        let arguments_needed = values.1;
                        *return_map.get_mut(&key.to_string()).unwrap() = (
                            true,
                            collected_raw_args[pos..(pos + arguments_needed)]
                                .iter()
                                .cloned()
                                .collect(),
                        );
                    }
                };
            }
        }
        dbg!(&return_map);
        return_map
    }
}

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

fn get_ascii(info: &OsInfo) -> String {
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
__
 _)
|
*
";
    let art_path = if &info.os_type == "linux" && &info.os_type == "freebsd" {
        path::Path::new("/etc/ascii-art")
    } else {
        path::Path::new("/etc/ascii-art")
    };
    let art: String;
    if path::Path::exists(art_path) {
        art = fs::read_to_string("/etc/ascii-art").unwrap();
    } else {
        art = if &info.os_type == "linux" {
            std_linux_art.to_string()
        } else if &info.os_type == "freebsd" {
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

    // get the longest art line
    let longest_art_line = &art_lines.iter().max().unwrap().len();
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
        let length = param.len();
        if length > lastlength {
            lastlength = length;
        };
    }

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
            tmp_fieldstrings.push(format!("┃{}┃", params[i]));
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

    let wait = if tmp_fieldstrings.len() > art_lines.len() {
        ((tmp_fieldstrings.len() as f32 / 2.0).floor() - (art_lines.len() as f32 / 2.0).floor())
            as usize
    } else {
        ((art_lines.len() as f32 / 2.0).floor() - (tmp_fieldstrings.len() as f32 / 2.0).floor())
            as usize
    };

    // create counter
    let mut wait_counter = wait.clone();

    // combine art and fields into one output
    for i in 0..out_length {
        let spaces_needed: usize;
        let artline = if wait_counter == 0 && i - wait < art_lines.len() {
            spaces_needed = longest_art_line - art_lines[i - wait].len();
            art_lines[i - wait].to_string()
        } else {
            spaces_needed = longest_art_line.to_owned();
            String::from("")
        };
        if wait_counter != 0 {
            wait_counter -= 1;
        };
        let fieldstring = if i < tmp_fieldstrings.len() {
            tmp_fieldstrings[i].as_str().to_string()
        } else {
            String::from("")
        };
        outstr.push_str(format!("  {}{:>spaces_needed$}  {}\n", artline, "", fieldstring).as_str());
    }

    outstr
}

fn main() {
    // debugging atm, will finish later
    let mut parser = Argument::new();
    parser.add_arg('f', "foo", 0);
    parser.add_arg('a', "all", 1);
    parser.add_arg(' ', "a-long-arg", 0);
    let args = parser.parse_args();
    dbg!(args);
    // start of program
    let info = OsInfo::new();
    let art = get_ascii(&info);
    let output = create_output(art, info);
    println!("{}", output);
}
