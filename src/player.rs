use std::process::Command;

pub fn ensure_player() {
    let status = Command::new("vlc").arg("--version").output();

    match status {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("VLC is not installed or not available");
                std::process::exit(1);
            }
        }

        Err(e) => {
            eprintln!("Failed to check VLC status: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn play_song(song_file: &str) {
    let status = Command::new("vlc").arg(song_file).spawn();

    match status {
        Ok(child) => {
            println!("VLC process started: {:?}", child.id());
        }

        Err(e) => {
            eprintln!("Failed to start VLC process: {}", e);
        }
    }
}
