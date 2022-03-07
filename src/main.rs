use std::process::Command;

fn whoami() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to run 'whoami'!");

    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn pwd() -> String {
    let output = Command::new("pwd")
        .arg("-L")
        .output()
        .expect("Failed to run 'pwd'!");

    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn shorten(path: String) -> String {
    let mut output = String::with_capacity(path.len());

    let home = format!("/home/{}", whoami());

    let mut skip = 0;

    if path.starts_with(&home) {
        output.push_str("~");
        // skip 3 components and not only 2, because the first one is the empty string
        skip = 3;
    }

    let last = path.split('/').count() - 1;
    for (index, component) in path.split('/').enumerate() {
        if index < skip || component.len() == 0 {
            continue;
        }

        output.push('/');
        if index == last {
            output.push_str(component);
        } else {
            if component.starts_with(".") {
                output.push('.');
                output.push(component.chars().nth(1).unwrap());
            } else {
                output.push(component.chars().nth(0).unwrap());
            }
        }
    }

    output
}

fn main() {
    println!("{}", shorten(pwd()));
}
