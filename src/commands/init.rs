use std::fs;

pub fn init_command() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
}
