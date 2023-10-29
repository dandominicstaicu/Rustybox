use std::f32::consts::E;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};

// https://doc.rust-lang.org/std/env/fn.current_dir.html
fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn echo(no_endl: bool, message: String) -> Result<(), i32>{
    if message.is_empty() {
        return Err(-10);
    }

    if no_endl {
        print!("{}", message);
    } else {
        println!("{}", message);
    }

    Ok(())
}

fn cat(filenames: Vec<String>) -> Result<(), i32> {
    for filename in &filenames {
        let file = File::open(filename);
        match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    match line {
                        Ok(l) => println!("{}", l),
                        Err(_) => return Err(-20),
                    }
                }
            }
            Err(_) => return Err(-20),
        }
    }
    Ok(())
}

fn mkdir(dirs_names: Vec<String>) -> Result<(), i32> {
    for dir_name in dirs_names {
        if let Err(_) = fs::create_dir(dir_name) {
            return Err(-30);
        }
    }
    Ok(())
    
}

// https://doc.rust-lang.org/std/fs/fn.rename.html
fn mv(source: &String, dest: &String) -> Result<(), i32> {
    if let Err(_) = fs::rename(source, dest) {
        return Err(-40);
    }
    Ok(())
}

// https://doc.rust-lang.org/stable/std/os/unix/fs/fn.symlink.html
fn ln(symbolic: bool, src: &str, link_name: &str ) -> Result<(), i32>{
    if symbolic {
        if let Err(_) = std::os::unix::fs::symlink(src, link_name) {
            return Err(-50);
        }
    } else {
        if let Err(_) = fs::hard_link(src, link_name) {
            return Err(-50);
        }
    }
    Ok(())
}

fn rmdir(dirs_names: Vec<String>) -> Result<(), i32> {
    for dir_name in dirs_names {
        if let Err(_) = fs::remove_dir(dir_name) {
            return Err(-60);
        }
    }
    Ok(())
}

fn rm(recursive: bool, dir: bool, names: Vec<String>) -> Result<(), i32> {
    let mut errors = Vec::new();

    for name in names {
        let path = Path::new(&name);
        if path.is_file() {
            if let Err(_) = fs::remove_file(&path) {
                errors.push(-70);
            }
        } else if path.is_dir() {
            if recursive {
                if let Err(_) = fs::remove_dir_all(&path) {
                    errors.push(-70);
                }
            } else if dir {
                if let Err(_) = fs::remove_dir(&path) {
                    errors.push(-70);
                }
            } else {
                // This is the expected error when neither `-d` nor `-r` is used
                errors.push(-70);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors[0])
    }
}


fn ls(all: bool, recursive: bool, path: &Path) {
    
}

fn cp(recursive: bool, src: String, dest: String) -> Result<(), i32> {
    let src_path = Path::new(&src);
    let dest_path = Path::new(&dest);
    
    if !src_path.exists() {
        return Err(-90);
    }

    if src_path.is_file() {
        // Copying a file
        if let Err(_) = fs::copy(&src_path, &dest_path) {
            return Err(-90);
        }
    } else if src_path.is_dir() && recursive {
        // Copying a directory recursively
        // Create the destination directory if it doesn't exist
        if !dest_path.exists() {
            if let Err(_) = fs::create_dir(&dest_path) {
                return Err(-90);
            }
        }

        for entry in fs::read_dir(&src_path).map_err(|_| -90)? {
            let entry = entry.map_err(|_| -90)?;
            let entry_path = entry.path();

            let dest_child_path = dest_path.join(entry_path.file_name().ok_or(-90)?);

            if entry_path.is_dir() {
                // Recursively copy the subdirectory
                cp(true, entry_path.to_string_lossy().to_string(), dest_child_path.to_string_lossy().to_string())?;
            } else if entry_path.is_file() {
                // Copy the file
                if let Err(_) = fs::copy(&entry_path, &dest_child_path) {
                    return Err(-90);
                }
            }
        }
    } else {
        return Err(-90);
    }

    Ok(())
}

fn touch(date_time: bool, no_creat: bool, modify: bool, name: String) {

}

fn chmod(r: i32, w: i32, x: i32, name: String) {

}

fn run() -> Result<(), i32> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();



    if args.len() > 1 {
        let command: &str = &args[1];
    
        match command {
            "pwd" => pwd().expect("Error executing pwd command!"),
            "echo" => {
                if args.len() < 3 {
                    // println!("Invalid command");
                    // return;
                }
                let result = if args[2] == "-n" {
                    echo(true, args[3..].join(" "))
                } else {
                    echo(false, args[2..].join(" "))
                };

                if let Err(exit_code) = result {
                    eprintln!("{}", 246);
                    std::process::exit(exit_code);
                }
            },
            "cat" => {
                if args.len() > 2 {
                    let filenames = args[2..].to_vec();
                    let result = cat(filenames);
                    if let Err(exit_code) = result {
                        eprintln!("{}", 236); // Displaying the error code as 236 as per your request.
                        std::process::exit(-20);

                    }
                } else {
                    print!("No filename provided for cat command!");
                }
            },
            "mkdir" => {
                if args.len() > 2 {
                    let dirnames = args[2..].to_vec();
                    let result = mkdir(dirnames);
                    if let Err(exit_code) = result {
                        eprintln!("{}", 226);
                        std::process::exit(exit_code);
                    }
                } else {
                    println!("Directory name not provided for mkdir command!");
                }
            },
            "mv" => {
                if args.len() > 2 {
                    let result = mv(&args[2], &args[3]);
                    if let Err(exit_code) = result {
                        eprintln!("{}", 216);
                        return Err(exit_code);
                    }
                } else {
                    println!("Source and destination not provided for mv command!");
                    return Err(-40);
                }

            },
            "ln" => {
                if args.len() > 2 {
                    let symbolic = args.contains(&String::from("-s")) || args.contains(&String::from("--symbolic"));
                    
                    let start_index = 2 + symbolic as usize;
                    if start_index >= args.len() {
                        println!("Source and link name not provided for ln command!");
                        return Err(-50);
                    }
                    
                    let src_index = &args[start_index];
                    let link_name = &args[start_index + 1];

                    let result = ln(symbolic, src_index, link_name);
                    if let Err(exit_code) = result {
                        // eprintln!("{}", 206);
                        return Err(-1);
                    }
                } else {
                    println!("Source and link name not provided for ln command!");
                    return Err(-50);
                }
            }
            "rmdir" => {
                if args.len() > 2 {
                    let dirnames = args[2..].to_vec();
                    let result = rmdir(dirnames);
                    if let Err(exit_code) = result {
                        eprintln!("{}", 196);
                        return Err(exit_code);
                    }
                } else {
                    println!("Directory name not provided for rmdir command!");
                    return Err(-60);
                }
            },
            "rm" => {
                let recursive = args.contains(&String::from("-R")) || args.contains(&String::from("--recursive")) || args.contains(&String::from("-r"));
                let dir = args.contains(&String::from("-d")) || args.contains(&String::from("--dir"));
            
                let start_idx = 2 + recursive as usize + dir as usize;

                if start_idx >= args.len() {
                    // eprintln!("File name not provided for rm command!");
                    return Err(-1);
                }

                let names = args[start_idx..].to_vec();
                let result = rm(recursive, dir, names);
                if let Err(exit_code) = result {
                    eprintln!("{}", 186);
                    return Err(exit_code);
                }
            },
            "ls" => {
                
            },
            "cp" => {
                let recursive = args.contains(&String::from("-R"))
                    || args.contains(&String::from("-r"))
                    || args.contains(&String::from("--recursive"));

                let start_index = 2 + recursive as usize;

                if start_index + 1 >= args.len() {
                    eprintln!("Source and dest are not provided for cp command!");
                    return Err(-90);
                }

                let src = &args[start_index];
                let dest = &args[start_index + 1];

                let result = cp(recursive, src.clone(), dest.clone());
                if let Err(exit_code) = result {
                    eprintln!("{}", 166);
                    return Err(exit_code);
                }
                
                
            },
            "touch" => println!("Matched 'touch' function!"),
            "chmod" => println!("Matched 'chmod' function!"),
            _ => {
                return Err(-1);
            },
        }

    } else {
        // TODO return -1;
        return Err(-1);
    }
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err_code) => {
            println!("Invalid command");
            err_code
        }
    });
}
