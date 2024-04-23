use std::process::Command;

fn main() {
    let current_dir = std::env::current_dir().unwrap();

    let paths = std::fs::read_dir(current_dir).unwrap();

    for path in paths {
        let entry = path.unwrap();
        let borrowed_path = entry.path();
        let current_path = borrowed_path.file_name().unwrap().to_str().unwrap();

        if entry.file_type().unwrap().is_dir() && current_path.starts_with("fudwud") {
            let output = Command::new("git")
                .arg("-C")
                .arg(&current_path)
                .arg("clean")
                .arg("-Xfd")
                .output()
                .expect("Failed to execute command");

            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
