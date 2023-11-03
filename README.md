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

### ls

Different behaviour for files and directories.
If the target is a directory, checks for flags and prints the entries inside it according to whata was was required; more details about this in comments in the code;
If the target is a file, print it's name.

### cp

If the source is a file, the copy is done by using fs::copy at the given destination;
If the source is a dir, it has to be recursive. It copies the contents of the src dir: if it's a file using fs::copy, if it's a dir by calling cp() recursively.

### touch

If it doesn't exist and is allowed to create it, the function uses File::create to create the new file;
In order to modify the access time of the file, it opens the file and reads from it;
In order to change the last modify time of the file, it opens it and writes a space;
If no_creat was set and the file doesn't exist, just return with no error code;

### chmod

Takes a string slice permission representing either numeric or symbolic permission modes, and a name as a String which is the path to the file or directory.

The function checks if the permission string can be parsed as an octal number. If successful, it sets the file permissions to this numeric value using fs::set_permissions. 

Handle symbolic permissions: If the permission string is not numeric, the function assumes it is symbolic. It retrieves the current metadata of the file and its mode.

For each permission chunk, it finds the index of the operation character (+ or -). It then determines the entities (u for user, g for group, o for others, a for all) and the permissions (r, w, x) to be modified. For each permission character, it calculates a permission mask.

It then applies the operation (+ to add permissions, - to remove them) to the current mode. After calculating the updated mode, the function applies it to the file or directory using fs::set_permissions. 

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