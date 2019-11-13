use std::env;
use std::io;
use std::path::Path;
use std::process;

fn find_rev_ctrl_root(dir: &Path) -> Result<&Path, io::Error> {
    let mut p = dir;
    loop {
        if p.join(".git").is_dir() {
            return Ok(p);
        }
        match p.parent() {
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No revision control root found",
                ))
            }
            Some(par) => {
                p = par;
            }
        }
    }
}

fn go() -> Result<process::ExitStatus, io::Error> {
    let cwd = Path::new(".").canonicalize()?;
    let root = find_rev_ctrl_root(&cwd)?;
    let args: Vec<String> = env::args().skip(1).collect();
    println!("Executing {:?} in {:?}", args, root);
    let es = process::Command::new(&args[0])
        .args(&args[1..])
        .current_dir(root)
        .spawn()
        .expect("Command failed")
        .wait()
        .expect("Command wasn't running");
    Ok(es)
}

fn main() {
    match go() {
        Ok(es) => process::exit(es.code().unwrap_or(1)),
        Err(e) => {
            println!("ERR: {:?}", e);
            process::exit(1);
        }
    }
}
