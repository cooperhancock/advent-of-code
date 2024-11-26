use std::fs;

pub fn input(file_path: &str) -> String {
    fs::read_to_string(String::from(file_path))
        .expect("Could not read from file")
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn input_from_file() {
        let result = input("./test_input.txt");
        assert_eq!(result, String::from("this is a test"));
    }
}
