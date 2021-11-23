use std::fs;
use std::io::Read;
use std::process::Command;

extern crate num_cpus;
fn main(){
    let filename = "/etc/os-release";
    let mut contents = String::new();

    let output = Command::new("sh").arg("-c").arg("uname -r")
                                .output().expect("Failed to execute command");

    let mut os_name = String::new();
    let mut os_like = String::new();
    let kernel = String::from_utf8(output.stdout).unwrap();

    match fs::File::open(filename){
        Result::Ok(mut file) => {
            file.read_to_string(&mut contents).unwrap();
        }
        Result::Err(err) => {
            println!("Error, {}", err);
        }
    }
    
    for line in contents.lines(){
        if line.contains("PRETTY_NAME"){
            let split_line = line.split('"');
            let split_vec: Vec<&str> = split_line.collect();
            os_name = split_vec[1].to_string();
        }
        if line.contains("ID_LIKE"){
            let split_line = line.split("=");
            let split_vec: Vec<&str> = split_line.collect();
            os_like = split_vec[1].to_string();
        }
    }

    println!("OS: {}\nBased on: {}\nKernel: {}\nLogical Cores: {}\nPhysical Cores: {}\n", 
        os_name, os_like, kernel, num_cpus::get(), num_cpus::get_physical());
}