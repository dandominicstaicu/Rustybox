use std::io::{BufRead, BufReader, Write, Read, self};
use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::fs::{OpenOptions, File};


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

        if !path.exists() {
            errors.push(-70);
            continue;
        }

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
        Err(-70)
    }
}

fn ls(all: bool, recursive: bool, path: impl AsRef<Path>) -> Result<(), i32> {
    let path = path.as_ref();

    if path.is_dir() {
        // If '-a' flag is set, print the '.' and '..' directories
        if all {
            println!(".");
            println!("..");
        }

        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => {
                eprintln!("Failed to read directory: {}", path.display());
                return Err(-80);
            }
        };

        for entry in entries {
            let entry = entry.map_err(|_| {
                eprintln!("Failed to read directory entry in: {}", path.display());
                -80
            })?;
            let file_name = entry.file_name().into_string().map_err(|_| -80)?;

            // Skip hidden files/directories if '-a' flag is not set
            if !all && file_name.starts_with('.') {
                continue;
            }

            println!("{}", file_name);

            if recursive && entry.path().is_dir() {
                println!("{}:", entry.path().display());
                ls(all, recursive, &entry.path())?;
            }
        }
    } else if path.is_file() {
        println!("{}", path.display());
    } else {
        eprintln!("Path does not exist: {}", path.display());
        return Err(-80);
    }

    Ok(())
}

fn cp(recursive: bool, src: &str, dest: &str) -> Result<(), i32> {
    let src_path = Path::new(src);
    let mut dest_path = PathBuf::from(dest);
    
    if !src_path.exists() {
        eprintln!("src_path does not exist");
        return Err(-90);
    }

    if src_path.is_file() {
        // If dest_path is a directory, append the filename to dest_path.
        if dest_path.is_dir() {
            dest_path.push(src_path.file_name().unwrap());
        }
        
        // Copying a file
        match fs::copy(src, &dest_path) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error copying file: {}", e);
                return Err(-90);
            }
        }
    } else if src_path.is_dir() && recursive {
        if dest_path.is_dir() {
            dest_path.push(src_path.file_name().ok_or(-90)?);
        }

        // Now, create the destination directory if it doesn't exist.
        if !dest_path.exists() {
            if let Err(e) = fs::create_dir_all(&dest_path) { 
                eprintln!("Error creating directory {}: {}", dest_path.display(), e);
                return Err(-90);
            }
        }

        for entry in fs::read_dir(&src_path).map_err(|_| -90)? {
            let entry = entry.map_err(|_| -90)?;
            let entry_path = entry.path();

            let dest_child_path = dest_path.join(entry_path.file_name().ok_or(-90)?);

            if entry_path.is_dir() {
                // Recursively copy the subdirectory
                cp(true, entry_path.to_str().ok_or(-90)?, dest_child_path.to_str().ok_or(-90)?)?;
            } else if entry_path.is_file() {
                // Copy the file
                if let Err(_) = fs::copy(&entry_path, &dest_child_path) {
                    eprintln!("Error copying file");
                    return Err(-90);
                }
            }
        }
    } else {
        eprintln!("Error copying directory");
        return Err(-90);
    }

    Ok(())
}

fn touch(a: bool, no_creat: bool, m: bool, name: String) -> Result<(), i32> {
    // The path to the file
    let path = Path::new(&name);

    // If the file exists, or we're allowed to create it
    if path.exists() || !no_creat {
        let mut edit_a_m: bool = false;
        if path.exists() {
            edit_a_m = true;
        }
        // If the file doesn't exist, create it.
        if !path.exists() {
            if let Err(_) = File::create(&path) {
                eprintln!("Error creating the file: {}", name);
                return Err(-100);
            }
        }

        

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("failed to open the file: {}", e);
                return Err(-100);
            }
        };

        if a || edit_a_m {
            
            let mut buffer = Vec::new();

            if let Err(e) = file.read_to_end(&mut buffer) {
                eprintln!("failed to read to end");
            }

        }

        if m || edit_a_m {
            let mut file = match OpenOptions::new().append(true).create(true).open(&path) {
                Ok(file) => file,
                Err(e) => {
                    println!("failed to open or create the file: {}", e);
                    return Err(-100);
                }
            };

            if let Err(e) = file.write_all(b" ") {
                println!("Failed to write to the file: {}", e);
            } else {
                println!("Space written to the file successfully!");
            }
        }
            
        drop(file);
        

        Ok(())
    } else if no_creat {
        Err(0)
    } else{
        eprintln!("File does not exist, and no-create option is set.");
        Err(-100)
    }
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
                    // std::process::exit(exit_code);
                    return Err(-10);
                }
            },
            "cat" => {
                if args.len() > 2 {
                    let filenames = args[2..].to_vec();
                    let result = cat(filenames);
                    if let Err(exit_code) = result {
                        eprintln!("{}", 236); // Displaying the error code as 236 as per your request.
                        // std::process::exit(-20);
                        return Err(-20);

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
                        // std::process::exit(exit_code);
                        return Err(-30);
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
                    let wrong = args.contains(&String::from("-a"));

                    if wrong {
                        // println!("Invalid option for ln command!");
                        return Err(-1);
                    }

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
                        return Err(-50);
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
                    return Err(-1); //-70
                }

                let names = args[start_idx..].to_vec();
                let result = rm(recursive, dir, names);
                if let Err(exit_code) = result {
                    eprintln!("{}", 186);
                    return Err(exit_code);
                }
            },
            "ls" => {
                let all = args.contains(&String::from("-a")) || args.contains(&String::from("-all"));
                let recursive = args.contains(&String::from("-R")) || args.contains(&String::from("--recursive"));
                
                let mut path_idx = 2;
                if all {
                    path_idx += 1;
                }

                if recursive {
                    path_idx += 1;
                }

                let path = if path_idx < args.len() {
                    &args[path_idx]
                } else {
                    "."
                };

                let result = ls(all, recursive, Path::new(path));
                if let Err(exit_code) = result {
                    eprintln!("{}", 176);
                    return Err(exit_code);
                }

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

                let result = cp(recursive, src, dest);
                if let Err(exit_code) = result {
                    eprintln!("result error: {}", 166);
                    return Err(exit_code);
                }
                
                
            },
            "touch" => {
                let a = args.contains(&String::from("-a"));
                let no_creat = args.contains(&String::from("-c")) || args.contains(&String::from("--no-creat"));
                let m = args.contains(&String::from("-m"));

                let start_idx = 2 + a as usize + no_creat as usize + m as usize;
                if start_idx >= args.len() {
                    eprintln!("File name not provided for touch command!");
                    return Err(-1);
                }

                let file_name = &args[start_idx];
                let result = touch(a, no_creat, m, file_name.to_string());
                if let Err(exit_code) = result {
                    eprintln!("{}", 156);
                    return Err(exit_code);
                }
            },
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