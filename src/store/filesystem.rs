use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::constants::filesystem_constants::DEFAULT_COMMANDS_DIRECTORY_PATH;

pub fn on_init() {
    create_command_directory();
    println!("Init successfully")
}

pub fn get_command_file(filename: &String) -> File {
    let f = OpenOptions::new()
        .read(true)
        .append(true)
        .truncate(false)
        .open(get_command_file_path(filename));

    match f {
        Ok(f) => {
            println!("Get file successfully");
            f
        }
        Err(err) => panic!("Failed to get file: {}", err),
    }
}

pub fn get_file_content(mut f: File) -> String {
    let mut strbuf = String::new();
    let result = f.read_to_string(&mut strbuf);
    match result {
        Ok(_) => {
            println!("Data readed successfully");
            strbuf
        }
        Err(err) => panic!("Failed to read data: {}", err),
    }
}

pub fn create_command_file(filename: &String) -> File {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(get_command_file_path(filename));

    match f {
        Ok(f) => {
            println!("File created");
            f
        }
        Err(err) => panic!("Failed to create file: {}", err),
    }
}

pub fn delete_command_file(filename: &String) {
    let result = fs::remove_file(get_command_file_path(&filename));
    match result {
        Ok(_) => println!("File deleted successfully"),
        Err(err) => panic!("Failed to write data: {}", err),
    }
}

pub fn write_to_file(mut f: File, command: String) {
    let result = f.write_all(command.as_bytes());

    match result {
        Ok(_) => println!("Data written successfully"),
        Err(err) => panic!("Failed to write data: {}", err),
    }
}

fn create_command_directory() {
    let is_exist = Path::new(DEFAULT_COMMANDS_DIRECTORY_PATH).is_dir();

    if is_exist {
        return;
    }

    let result = fs::create_dir(DEFAULT_COMMANDS_DIRECTORY_PATH);

    match result {
        Ok(_) => println!("Directory created successfully"),
        Err(err) => panic!("Failed to create directory: {}", err),
    }
}

fn get_command_file_path(filename: &String) -> String {
    let command_file_path = format!("./{}/{}.txt", DEFAULT_COMMANDS_DIRECTORY_PATH, filename);

    command_file_path
}
