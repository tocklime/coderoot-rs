extern crate clap;

use std::io;
use std::path::Path;
use std::process;

fn find_rev_ctrl_root(dir: &Path) -> Result<&Path, io::Error> {
    let mut p = dir;
    loop {
        if p.join(".git").is_dir() || p.join(".svn").is_dir() {
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

fn go(cmd: Option<&str>, args: Vec<&str>, subdir: &str) -> Result<i32, io::Error> {
    let cwd = dunce::canonicalize(Path::new("."))?;
    let root = find_rev_ctrl_root(&cwd)?;
    let target_dir = dunce::canonicalize(root.join(Path::new(subdir)))?;
    let es = match cmd {
        None => {
            println!("{}", target_dir.display());
            0
        }
        Some(c) => {
            println!("Executing {:?} {:?} in {:?}", cmd, args, target_dir);
            process::Command::new(c)
                .args(&args)
                .current_dir(root)
                .spawn()
                .expect("Command failed")
                .wait()
                .expect("Command wasn't running")
                .code()
                .unwrap_or(1)
        }
    };
    Ok(es)
}

fn main() {
    let matches = clap::App::new("Code Root")
        .version("0.1")
        .author("Greg Manning <greg@gregmanning.uk>")
        .about("Run command in nearest parent revision control root directory")
        //.setting(clap::AppSettings::AllowLeadingHyphen)
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(
            clap::Arg::with_name("subdir")
                .short("s")
                .long("subdir")
                .value_name("SUBDIR")
                .help("Run in subdir of revision control root")
                .takes_value(true),
        )
        .arg(clap::Arg::with_name("cmd"))
        .arg(clap::Arg::with_name("cmd_args").multiple(true))
        .get_matches();

    let subdir = matches.value_of("subdir").unwrap_or("");
    let cmd = matches.value_of("cmd");
    let args = matches.values_of("cmd_args").unwrap_or_default().collect();
    println!("SD: {:?}\nCMD: {:?}\nARGS: {:?}", subdir, cmd, args);
    match go(cmd, args, subdir) {
        Ok(es) => process::exit(es),
        Err(e) => {
            println!("ERR: {:?}", e);
            process::exit(1);
        }
    }
}
