use std::process::Command;

pub fn invoke(body: String, path: String) {
    println!("body: {0} , path: {1} \n", body, path);

    let output = Command::new(body)
        .current_dir(path)
        .output()
        .expect("Failed to execute command");

    let mut log = String::new();

    log.push_str(match std::str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from git"),
    });

    println!("command output:\n{0}", log,)
}
