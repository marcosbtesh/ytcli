use std::process::Command;

pub fn play(url: &str) {
    let _ = Command::new("mpv").arg(url).status();
}
