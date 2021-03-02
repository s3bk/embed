use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let input = "DATA";

    Command::new("cp")
        .current_dir(&out)
        .arg(input)
        .arg("EMBEDDED_DATA")
        .output()
        .unwrap();
    
    Command::new("ld")
        .current_dir(&out)
        .args(&["-r", "-b", "binary", "-o"])
        .arg("EMBEDDED_DATA.o")
        .arg("EMBEDDED_DATA")
        .output()
        .unwrap();
        
    Command::new("ar")
        .current_dir(&out)
        .args(&["r", "EMBEDDED_DATA.a", "EMBEDDED_DATA.o"])
        .output()
        .unwrap();

    println!("rustc-link-search=static={}", out.to_str().unwrap());
    println!("rustc-link-lib=static=EMBEDDED_DATA.a");
}