use std::{collections::HashMap, env, fs, path, process::exit, str};
use uname::uname;

struct Argument {
    name: String,
    description: String,
    exit_statuses: HashMap<u8, String>,
    epilog: String,
    credits: String,
    args: (HashMap<String, (String, isize)>, HashMap<char, (String, isize, String)>),
    help_order: (Vec<String>, Vec<String>)
}

impl Argument {
    fn new(name: &str, description: &str, epilog: &str, credits: &str) -> Self {
        let mut args: (HashMap<String, (String, isize)>, HashMap<char, (String, isize, String)>) =
            (HashMap::new(), HashMap::new());
        let exit_statuses: HashMap<u8, String> = HashMap::new();
        let mut help_order: (Vec<String>, Vec<String>) = (vec![], vec![]);
        args.1.insert('h', ("help".to_string(), 0, "Use this to print this help message".to_string()));
        help_order.1.push('h'.to_string());
        Self {
            name: name.to_string(),
            description: description.to_string(),
            exit_statuses,
            epilog: epilog.to_string(),
            credits: credits.to_string(),
            args,
            help_order
        }
    }

    fn add_arg(&mut self, placeholder: &str, args: &str, help: Option<&str>) {
        let nargs = if args == "+" {
            -1
        } else {
            match args.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    eprintln!("Error! \"args\" parameter must be either a positive integer, 0 or +");
                    exit(1);
                }
            }
        };
        self.help_order.0.push(placeholder.to_string());
        self.args.0.insert(placeholder.to_string(), (help.unwrap_or("").to_string(), nargs));
    }

    fn add_option(&mut self, mut short: char, long: &str, parameters: &str, help: Option<&str>) {
        if short == ' ' {
            short = '-'
        }; 

        let nargs = if parameters == "+" {
            -1
        } else {
            match parameters.to_string().parse::<usize>() {
                Ok(n) => n as isize,
                Err(_) => {
                    eprintln!("Error! \"parameters\" parameter must be either a positive integer, 0 or +");
                    exit(1);
                }
            }
        };
        
        if short == '-' {
            self.help_order.1.push(long.to_string());
        } else {
            self.help_order.1.push(short.to_string());
        };

        self.args.1.insert(short, (long.to_string(), nargs, help.unwrap_or("").to_string()));
    }

    fn print_help(&self) {
        let mut help_string = String::new();
        let options = &self.args.1;
        let pos_args = &self.args.0;
        let name = &self.name;
        let description = &self.description;
        let credits = &self.credits;
        let bottom_text = &self.epilog;
        let exit_statuses = &self.exit_statuses;
        let usage = format!("usage: {}", name);
        let help_orders = &self.help_order;
        help_string.push_str(format!("{}\n{}\n\nOptions:\n", description, usage).as_str());

        for option in &help_orders.1 {
            let key: char;
            let mut field = (&' ', &(String::new(), 0isize, String::new())); 
            if option.len() > 1 {
                let mut found = false;
                for (tempkey, tempvalues) in &*options {
                    if tempvalues.0 == option.to_owned() {
                        field = (tempkey, tempvalues);
                        found = true;
                        break
                    }
                }
                if found == false {
                    eprintln!("Exception, couldn't get order of value in help message");
                    exit(1);
                }
            } else {
                field = options.get_key_value(&option.chars().nth(0).unwrap()).unwrap()
            };
            if field.0.to_owned() == '-' {
                key = ' ';
            } else {
                key = field.0.to_owned();
            };
            let values = field.1;
            help_string.push_str(format!("{}{}\t--{}\t\t{}\n", if key == ' '{ "" } else { "-" }, key, values.0, values.2).as_str());
        }

        if exit_statuses.len() > 1 {
            for (key, value) in &*exit_statuses {
                help_string.push_str(format!("Exit Statuses:\n\t{}\t{}\n", key, value).as_str());
            }
        };
        help_string.push_str(format!("\n{}\n{}\n", bottom_text, credits).as_str());

        println!("{}", help_string);
    }

    fn parse_args(&mut self) -> HashMap<String, (bool, Vec<String>)> {
        let raw_args = std::env::args();
        let mut collected_raw_args: Vec<String> = std::env::args().collect();
        collected_raw_args.remove(0);
        let options = &self.args.1;
        let mut return_map: HashMap<String, (bool, Vec<String>)> = HashMap::new();
        for (key, val) in options.iter() {
            let name: String;
            if key.to_owned() == '-' {
                name = val.0.to_owned();
            } else {
                name = key.to_string();
            };
            return_map.insert(name, (false, vec![]));
        }
        for (pos, argument) in raw_args.into_iter().enumerate() {
            if pos == 0 {
                continue;
            };
            if argument.len() > 1
                && argument.starts_with("-")
                && argument.chars().nth(1).unwrap() != '-'
            {
                for part in argument.get(1..).unwrap().chars() {
                    if options.contains_key(&part) {
                        let options_needed = options.get(&part).unwrap().1;
                        if collected_raw_args.len() < pos + options_needed as usize {
                            eprintln!("Too few options passed to -{}", &part);
                            exit(1);
                        }
                        *return_map.get_mut(&part.to_string()).unwrap() = (
                            true,
                            collected_raw_args[pos..(pos + options_needed as usize)]
                                .iter()
                                .cloned()
                                .collect(),
                        );
                    };
                }
            } else if argument.len() > 2 && argument.get(..2).unwrap() == "--" {
                let part = argument.get(2..).unwrap();
                for (key, values) in &*options {
                    if part == values.0 {
                        let name: String;
                        if key.to_owned() != '-' {
                            name = key.to_string();
                        } else {
                            name = part.to_string();
                        };
                        let options_needed = values.1;
                        *return_map.get_mut(&name).unwrap() = (
                            true,
                            collected_raw_args[pos..(pos + options_needed as usize)]
                                .iter()
                                .cloned()
                                .collect(),
                        );
                    }
                }
            }
        }
        if return_map.get("h").unwrap().0 == true {
            self.print_help();
            exit(0);
        };
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
    let mut parser = Argument::new(
        "fetch",
        "A fetch tool written in Rust for Linux, Windows, MacOS and BSD",
        "The bottom text :O",
        "SpamixOfficial 2023",
    );
    parser.add_arg("THE_PLACEHOLDER", "+", Some("PLACEHOLDER ARGUMENT"));
    parser.add_option('f', "foo", "0", Some("foo help"));
    parser.add_option('a', "all", "1", Some("all help"));
    parser.add_option(' ', "a-long-arg", "0", None);
    let args = parser.parse_args();
    dbg!(args);
    // start of program
    let info = OsInfo::new();
    let art = get_ascii(&info);
    let output = create_output(art, info);
    println!("{}", output);
}
