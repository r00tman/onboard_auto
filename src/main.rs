use std::io::{BufReader, BufRead};
use std::process::{Command, Child, Stdio};

fn get_device_path() -> Option<String> {
    let child = Command::new("libinput").arg("list-devices")
        .stdout(Stdio::piped())
        .spawn().expect("can't spawn libinput list-devices");

    let out = child.stdout.expect("can't open child's stdout");
    let br = BufReader::new(out);

    let mut found_one = false;
    for line in br.lines() {
        let l = line.unwrap();
        if l.contains("ThinkPad Extra Buttons") {
            found_one = true;
        } else if found_one {
            return Some(l.split(' ').next_back().unwrap().to_string());
        }
    }
    None
}

fn main() {
    let path = get_device_path()
        .expect("can't find thinkpad extra buttons device");

    let child = Command::new("stdbuf").arg("-o").arg("0")
        .arg("libinput").arg("debug-events").arg(path.as_str())
        .stdout(Stdio::piped())
        .spawn().expect("can't spawn libinput debug-events");

    let out = child.stdout.expect("can't open child's stdout");
    let br = BufReader::new(out);

    let mut onboard: Option<Child> = None;
    for line in br.lines() {
        let u = line.unwrap();
        let mut l: Vec<_> = u.split('\t').collect();

        match l.pop() {
            Some("switch tablet-mode state 1") => {
                if onboard.is_none() {
                    onboard = Command::new("onboard")
                        .stdout(Stdio::piped()).spawn().ok();
                }
            },
            Some("switch tablet-mode state 0") => {
                if let Some(ref mut x) = onboard {
                    x.kill().unwrap();
                }
            },
            _ => { }
        }
    }
}
