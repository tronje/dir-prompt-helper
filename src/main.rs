use std::env;
use std::io::{self, Write};
use std::path::{Path, MAIN_SEPARATOR_STR};

// Catch-all error to avoid allocations in Box<dyn Error>
#[derive(Debug)]
struct StaticError;

impl<E: std::error::Error> From<E> for StaticError {
    fn from(_: E) -> Self {
        Self
    }
}

type Result<T> = std::result::Result<T, StaticError>;

fn print_short_pwd<W>(pwd: &Path, home: &str, out: &mut W) -> Result<()>
where
    W: Write,
{
    let final_idx = pwd.iter().count() - 1;

    let mut skip = 0;

    if pwd.starts_with(home) {
        out.write_all(b"~")?;
        if final_idx >= 3 {
            out.write_all(b"/")?;
        }

        // skip /home/<username>, which is three path components (root dir, home, and <username>)
        skip = 3;
    }

    for (idx, component) in pwd.iter().enumerate().skip(skip) {
        let s = component.to_str().unwrap();

        if idx == final_idx {
            out.write_all(s.as_bytes())?;
        } else {
            if s == MAIN_SEPARATOR_STR {
                // do nothing
            } else if s.starts_with('.') {
                out.write_all(s[0..2].as_bytes())?;
            } else {
                out.write_all(s[0..1].as_bytes())?;
            }

            out.write_all(b"/")?;
        }
    }

    out.write_all(b"\n")?;
    out.flush()?;
    Ok(())
}

fn run() -> Result<()> {
    let pwd = env::current_dir()?;
    let home = env::var("HOME")?;
    let mut stdout = io::stdout().lock();
    print_short_pwd(&pwd, &home, &mut stdout)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(_) => println!("???"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_helper(path: &Path, home: &str) -> String {
        let mut out = Vec::new();
        print_short_pwd(path, home, &mut out).unwrap();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn root() {
        let path = PathBuf::from("/");
        let home = "/home/alice";
        let result = test_helper(&path, home);
        assert_eq!(&result, "/\n");
    }

    #[test]
    fn home() {
        let path = PathBuf::from("/home/alice");
        let home = "/home/alice";
        let result = test_helper(&path, home);
        assert_eq!(&result, "~\n");
    }

    #[test]
    fn foreign_home() {
        let path = PathBuf::from("/home/bob");
        let home = "/home/alice";
        let result = test_helper(&path, home);
        assert_eq!(&result, "/h/bob\n");
    }

    #[test]
    fn somewhere_outside_home() {
        let path = PathBuf::from("/alpha/bravo/charlie/delta/echo/foxtrott");
        let home = "/home/doesnt_matter";
        let result = test_helper(&path, home);
        assert_eq!(&result, "/a/b/c/d/e/foxtrott\n");
    }

    #[test]
    fn somewhere_in_home() {
        let path = PathBuf::from("/home/alice/alpha/bravo/charlie");
        let home = "/home/alice";
        let result = test_helper(&path, home);
        assert_eq!(&result, "~/a/b/charlie\n");
    }

    #[test]
    fn hidden_components() {
        let path = PathBuf::from("/alpha/.bravo/charlie/.delta/echo");
        let home = "/home/doesnt_matter";
        let result = test_helper(&path, home);
        assert_eq!(&result, "/a/.b/c/.d/echo\n");
    }
}
