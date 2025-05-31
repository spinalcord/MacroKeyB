use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::process;
use fs2::FileExt as _; // bringt lock_exclusive()

fn main() {
    // Allow only 1 instance
    let lock_path = "/tmp/macrokeyb.lock";
    let lock_file = File::create(lock_path).expect("Error creating lock file.");

    if let Err(_) = lock_file.try_lock_exclusive() {
        eprintln!("Application already running!");
        process::exit(1);
    }

    #[cfg(unix)]
    if !is_root() {
        eprintln!("Use: 'sudo -E ...'");
        process::exit(1);
    }

    macrokeyb_lib::run();
}

#[cfg(unix)]
fn is_root() -> bool {
    use std::process::Command;

    let output = Command::new("id")
        .arg("-u")
        .output()
        .expect("Error 'id -u'");

    let uid = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u32>()
        .expect("Cannot parse UID");

    uid == 0
}
