
const BASE_PATH: &str = "/home/ajay/RustProjects/COMPILER_API/c-cpp-rust/data";

pub mod c {
    use std::{fs::File, io::Write};
    use std::io::Error;
    use std::process::Command;
    use std::sync::{Mutex, Arc};

    use super::BASE_PATH;

    pub fn write_in_file(data: String) -> Result<(), Error> {
        let file = Arc::new(Mutex::new(File::create(format!("{}/c/temp.c",BASE_PATH)).unwrap()));
        
        let x = file.lock().unwrap().write_all(data.as_bytes());
        file.lock().unwrap().sync_all().unwrap();
        match x {
            Ok(_) => {
                println!("Program is written in file /data/c/temp.c");
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn compile() -> Result<(), Error> {
        let out = Command::new("gcc")
                                                .arg("temp.c")
                                                .current_dir(format!("{}/c",BASE_PATH))
                                                .output();

        match out {
            Ok(_) => {
                let c = out.unwrap();
                if !c.stdout.is_empty() {
                    return Err(Error::new(std::io::ErrorKind::Other, String::from_utf8_lossy(&c.stderr).to_string()));
                }
                Ok(())
            }
            Err(_) => {
                Err(Error::new(std::io::ErrorKind::Other, "Something went wrong during execution of:\n\t\tgcc temp.c".to_string()))
            }
        }
    }

    pub fn execute(data: String) -> String {
        let file = Arc::new(Mutex::new(File::create(format!("{}/c/in.log",BASE_PATH)).unwrap()));
        
        let x = file.lock().unwrap().write_all(data.as_bytes());
        match x {
            Ok(_) => {
                println!("Input is written in file /data/c/in.log");
            }
            Err(_) => {
                return "Something went wrong while parsing input".to_string().to_owned()
            }
        }
        
        let out  = Command::new("./run.sh")
                                                    .current_dir(format!("{}/c", BASE_PATH))
                                                    .output();
        match out {
            Ok(_) => {
                let c = out.unwrap();
                if c.stdout.is_empty() {
                    return String::from_utf8_lossy(&c.stderr).to_string();
                }
                let out = String::from_utf8_lossy(&c.stdout).to_string();
                println!("{}",out);
                return out;
            }
            Err(_) => {
                "Something went wrong during execution of:\n\t\t./temp.c".to_string().to_owned()
            }
        }
    }

}

pub mod cpp {
    use std::{fs::File, io::Write};
    use std::io::Error;
    use std::process::Command;
    use std::sync::{Mutex, Arc};

    use super::BASE_PATH;

    pub fn write_in_file(data: String) -> Result<(), Error> {

        let file = Arc::new(Mutex::new(File::create(format!("{}/cpp/temp.cpp",BASE_PATH)).unwrap()));
        
        let x = file.lock().unwrap().write_all(data.as_bytes());

        match x {
            Ok(_) => {
                println!("Program is written in file /data/cpp/temp.cpp");
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn compile() -> Result<(), Error> {
        let out = Command::new("g++")
                                                .arg("temp.cpp")
                                                .current_dir(format!("{}/cpp",BASE_PATH))
                                                .output();

        match out {
            Ok(_) => {
                let c = out.unwrap();
                if c.stdout.is_empty() {
                    return Err(Error::new(std::io::ErrorKind::Other, String::from_utf8_lossy(&c.stderr).to_string()));
                }
                Ok(())
            }
            Err(_) => {
                Err(Error::new(std::io::ErrorKind::Other,"Something went wrong during execution of:\n\t\tg++ temp.cpp".to_string()))
            }
        }
    }

    pub fn execute(data: String) -> String {
        let file = Arc::new (Mutex::new(File::create(format!("{}/cpp/in.log",BASE_PATH)).unwrap()));
        
        let x = file.lock().unwrap().write_all(data.as_bytes());

        match x {
            Ok(_) => {
                println!("input is written in file /data/cpp/in.log");
            }
            Err(_) => {
                "Something went wrong while parsing input";
            }
        }
        let out = Command::new("./run.sh")
                                                .current_dir(format!("{}/cpp",BASE_PATH))
                                                .output();

        match out {
            Ok(_) => {
                let c = out.unwrap();
                if c.stdout.is_empty() {
                    return String::from_utf8_lossy(&c.stderr).to_string();
                }
                let out = String::from_utf8_lossy(&c.stdout).to_string();
                println!("{}",out);
                return out;
            }
            Err(_) => {
                "Something went wrong during execution of:\n\t\t./temp.c".to_string()
            }
        }
    }

}

pub mod rust {
    use std::{fs::File, io::Write};
    use std::io::Error;
    use std::process::Command;
    use std::sync::{Mutex, Arc};

    use super::BASE_PATH;

    pub fn write_in_file(data: String) -> Result<(), Error> {
        let file = Arc::new(Mutex::new(File::create(format!("{}/rust/temp.rs",BASE_PATH)).unwrap()));
        
        let x = file.lock().unwrap().write_all(data.as_bytes());
        file.lock().unwrap().sync_all().unwrap();
        match x {
            Ok(_) => {
                println!("Program is written in file /data/rust/temp.rs");
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn compile() -> Result<(), Error> {
        let out = Command::new("rustc")
                                                .arg("temp.rs")
                                                .current_dir(format!("{}/rust",BASE_PATH))
                                                .output();

        match out {
            Ok(_) => {
                let c = out.unwrap();
                if c.stdout.is_empty() {
                    return Err(Error::new(std::io::ErrorKind::Other,String::from_utf8_lossy(&c.stderr).to_string()));
                }
                Ok(())
            }
            Err(_) => {
                Err(Error::new(std::io::ErrorKind::Other, "Something went wrong during execution of:\n\t\trustc temp.rs".to_string()))
            }
        }
    }

    pub fn execute(data: String) -> String {
        let mut file = File::create(format!("{}/rust/in.log",BASE_PATH)).unwrap();
        
        let x = file.write_all(data.as_bytes());
        match x {
            Ok(_) => {
                println!("Input is written in file /data/rust/in.log");
            }
            Err(_) => {
                return "Something went wrong while parsing input".to_string().to_owned()
            }
        }
        
        let out  = Command::new("./run.sh")
                                                    .current_dir(format!("{}/rust", BASE_PATH))
                                                    .output();
        match out {
            Ok(_) => {
                let c = out.unwrap();
                if c.stdout.is_empty() {
                    return String::from_utf8_lossy(&c.stderr).to_string();
                }
                let out = String::from_utf8_lossy(&c.stdout).to_string();
                println!("{}",out);
                return out;
            }
            Err(_) => {
                "Something went wrong during execution of:\n\t\t./temp.c".to_string().to_owned()
            }
        }
    }
    
}