use std::{
    env, fs,
    path::{self, Path},
};
use uname::uname;

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
    let mut art: String;
    if path::Path::exists(art_path) {
        art = fs::read_to_string("/etc/ascii-art").unwrap();
    } else {
        art = if &info.os_type == "linux" { std_linux_art.to_string() } else if &info.os_type == "freebsd" { std_freebsd_art.to_string() } else { std_unknown_art.to_string() };
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
    let info = OsInfo::new();
    let art = get_ascii(&info);
    let output = create_output(art, info);
    println!("{}", output);
}
