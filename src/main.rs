use std::{env, fs};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

fn rm(recursive: bool, dir: bool, names: String) {

}

fn ls(all: bool, recursive: bool, dir: String) {

}

fn cp(recursive: bool, src: String, dest: String) {

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
                if args.len() > 3 {
                    let symbolic = args.contains(&String::from("-s")) || args.contains(&String::from("--symbolic"));
                    let src_index = if symbolic { args.iter().position(|arg| arg != "-s" && arg != "--symbolic").unwrap() } else { 2 };
                    let link_name_index = src_index + 1;
                    let result = ln(symbolic, &args[src_index], &args[link_name_index]);
                    if let Err(exit_code) = result {
                        eprintln!("{}", 206);
                        return Err(exit_code);
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
            "rm" => println!("Matched 'rm' function!"),
            "ls" => println!("Matched 'ls' function!"),
            "cp" => println!("Matched 'cp' function!"),
            "touch" => println!("Matched 'touch' function!"),
            "chmod" => println!("Matched 'chmod' function!"),
            _ => println!("Command not recognized!"),
        }
    
    } else {
        // println!("Invalid command");
        // TODO return -1;
        return Err(-1);
    }
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err_code) => {
            eprint!("Invalid command");
            err_code
        }
    });
}
