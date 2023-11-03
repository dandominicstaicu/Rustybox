# Rustybox
Dan-Dominic Staicu 321CA 2023

## Overview

RustyBox implements basic Linux commands in Rust

## Commands Implementation

Main function's only purpose is to handle the error code and return it if it occured in the run() function.

The run() function gets the command line arguments and handles them;
it matches the command with known strings and processes the arguments depending on the inputed command.
Inside the match arms it also calls the function and handles it's error (if any occured).

All the commands are written as functions that return a Result<(), i32>:

### pwd

Gets the current directory and print's its display;

### cat

Gets a filenames array. It iterates through them , opens them, reads them, and print their output;

### mkdir

Gets an array of dir names, iterates through it and creates them;

### mv

Gets a source and a destination; it uses the fs::rename function from rust to change the file's path

### ln

Can create a symbolic link or a hard link of a file using Rust functions from std::os::unix::fs;

### rmdir

Gets an array of names, iterates through it and calls the Rust function to remove them;

### rm

Iterates through the given names and transforms them to a path;
if it is a file, it calls the fs::remove_file Rust function;
if it's a dir and the -r recursive parameter is set, it calls the fs::remove_dir_all Rust function;
if it's a dir, but -r is false, irt calls the fs::remove_dir Rust function;

If any error occures the err is set to true, and after checking all the names the error is returned; this is so it lets the other rms execute succesfully if one failed, but still return an error code;

## Requirement

https://upb-cs-rust.github.io/teme/rustybox.html

## Verify

Run the following commands to test your homework:

You will have to install NodeJS (it is installed in the codespace)

```bash
# Clone tests repository
git submodule update --init 

# Update tests repository to the lastest version
cd tests
git pull 
cd ..

# Install loadash
npm install lodash
```

Install rustybox

```bash
cargo install --path .
```

Run tests

```bash
cd tests
# Run all tests 
./run_all.sh

# Run single test
./run_all.sh pwd/pwd.sh
```