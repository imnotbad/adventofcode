pub mod input {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn into_string(filename: &str) -> String {
        let mut f = File::open(filename).expect("Error opening file");
        let mut data = String::new();
        f.read_to_string(&mut data).expect(
            "Error while reading file",
        );
        data
    }

    pub fn into_vec(filename: &str) -> Vec<u8> {
        let mut f = File::open(filename).expect("Error opening file");
        let mut data = Vec::new();
        f.read_to_end(&mut data).expect("Error while reading file");
        data
    }
}
