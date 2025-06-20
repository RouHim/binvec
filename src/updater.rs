use std::path::PathBuf;
use std::time::Duration;
use std::{env, io, process, thread};

use self_update::cargo_crate_version;

/// Checks for updates
/// If an update is available, download and install it
/// If no update is available, do nothing
/// Automatically restart the application after update
pub fn update() {
    // In release mode, don't ask for confirmation
    let no_confirm: bool = !cfg!(debug_assertions);

    let current_executable = env::current_exe().expect("current exe");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("rouhim")
        .repo_name("binvec")
        .bin_name("binvec")
        .show_download_progress(true)
        .no_confirm(no_confirm)
        .current_version(cargo_crate_version!())
        .build()
        .unwrap()
        .update();

    match status {
        Err(err) => println!("Failed to update: {}", err),
        Ok(self_update::Status::UpToDate(version)) => {
            println!("binvec {} is up to date", version);
        }
        Ok(self_update::Status::Updated(version)) => {
            println!("binvec updated to {}", version);
            restart_process(current_executable);
        }
    }
}

/// Restarts the current process
fn restart_process(current_executable: PathBuf) {
    println!("Waiting 3s before restarting {:?} ...", current_executable);
    thread::sleep(Duration::from_secs(3));
    let err = exec(process::Command::new(current_executable.clone()).args(env::args().skip(1)));
    eprintln!(
        "Error: Failed to restart process {:?}: {}",
        current_executable, err
    );
    std::process::exit(1);
}

/// Replaces the current process with a new one
/// This function is only available on Unix platforms
#[cfg(unix)]
fn exec(command: &mut process::Command) -> io::Error {
    use std::os::unix::process::CommandExt as _;
    // Completely replace the current process image. If successful, execution
    // of the current process stops here.
    command.exec()
}

#[cfg(windows)]
fn exec(command: &mut process::Command) -> io::Error {
    // On Windows, we cannot replace the current process, so we just spawn a new one
    // and exit the current one.
    match command.spawn() {
        Ok(_) => std::process::exit(0), // Exit successfully, never returns
        Err(err) => err,
    }
}