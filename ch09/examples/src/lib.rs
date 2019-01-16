#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::ErrorKind;
    use super::*;
    
    #[test]
    #[should_panic(expected = "Problem openning file")]
    fn it_will_panic_on_file_not_found() {
        let _f = File::open("not.found");
        let _f = match _f {
            Ok(file) => file,
            Err(error) => panic!("Problem openning file: {}", error),
        };
    }
    
    #[test]
    #[should_panic(expected = "File not found")]
    fn it_will_panic_with_file_not_found() {
        let _f = File::open("not.found");
        let _f = match _f {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound
                => panic!("File not found"),
            Err(error) => panic!("Something happen! {}", error),
        };
    }
    
    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn it_will_panic_on_unwrap_file_not_found() {
        let _f = File::open("not.found").unwrap();
    }
    
    #[test]
    #[should_panic(expected = "Errors")]
    fn it_will_propagate_errors() {
        let _f = file_not_found();
        let _f = match _f {
            Ok(file) => file,
            Err(error) => panic!("Errors"),
        };
    }
}

use std::io;
use std::io::Read;
use std::fs::File;

fn file_not_found() -> Result<String, io::Error> {
    let mut f = File::open("not.found")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
