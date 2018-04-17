use std::process::Command;


fn whoami() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to run 'whoami'!");

    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .to_owned()
}


fn pwd() -> String {
    let output = Command::new("pwd")
        .arg("-L")
        .output()
        .expect("Failed to run 'pwd'!");

    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .to_owned()
}


fn build_string() -> String {
    let mut pwd = pwd();

    let home = format!("/home/{}", whoami());

    if pwd.starts_with(&home) {
        pwd = pwd.replace(&home, "~");
    }

    pwd = pwd.split('/').map(|dir| {
        if pwd.ends_with(dir) {
            String::from("/") + dir

        } else if dir.starts_with(".") {
            ['/', dir.chars().nth(0).unwrap(), dir.chars().nth(1).unwrap()]
                .into_iter().collect::<String>()
        } else {
            match dir.chars().nth(0) {
                Some(c) => ['/', c].into_iter().collect::<String>(),
                None => String::from(""),
            }
        }
    }).collect::<String>();

    if pwd.starts_with("/~") {
        pwd = pwd.replace("/~", "~");
    }

    pwd.replace("//", "/")
}


fn main() {
    println!("{}", build_string());
}
