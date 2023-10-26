use std::env;
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

//TODO implement cat
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

fn mkdir(dirs_names: String) {

}

fn mv(source: String, dest: String) {

}

fn ln(symbolic: bool, src: String, link_name: String) {

}

fn rmdir(dirs_names: String) {

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

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();



    if args.len() > 1 {
        let command: &str = &args[1];
    
        match command {
            "pwd" => pwd().expect("Error executing pwd command!"),
            "echo" => {
                if args.len() < 3 {
                    // println!("Invalid command");
                    return;
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
            "mkdir" => println!("Matched 'mkdir' function!"),
            "mv" => println!("Matched 'mv' function!"),
            "ln" => println!("Matched 'ln' function!"),
            "rmdir" => println!("Matched 'rmdir' function!"),
            "rm" => println!("Matched 'rm' function!"),
            "ls" => println!("Matched 'ls' function!"),
            "cp" => println!("Matched 'cp' function!"),
            "touch" => println!("Matched 'touch' function!"),
            "chmod" => println!("Matched 'chmod' function!"),
            _ => println!("Command not recognized!"),
        }
    
    } else {
        println!("Invalid command");
        // TODO return -1;
    }

}
