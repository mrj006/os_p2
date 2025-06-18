pub mod fibonacci;
pub mod createfile;
pub mod deletefile;
pub mod reverse;
pub mod toupper;
pub mod random;
pub mod timestamp;
pub mod hash;
pub mod simulate;
pub mod sleep;
pub mod help;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn file() {
        let name = "test1";
        let content = "Hello World!\n";
        let repeat: u64 = 5;

        createfile::createfile(name, content, repeat).unwrap();

        let file = fs::OpenOptions::new()
            .read(true)
            .open(name)
            .unwrap();

        assert_eq!(content.len() as u64 * repeat, file.metadata().unwrap().len());

        std::fs::remove_file(name).unwrap();
    }

    #[test]
    #[should_panic]
    fn createfile_error() {
        let name = "test";
        let content = "Hello World!\n";
        let repeat: u64 = 5;

        createfile::createfile(name, content, repeat).unwrap();
        let result = createfile::createfile(name, content, repeat);

        if let Err(_) = result {
            let _ = deletefile::deletefile(name);
            panic!();
        }
    }

    #[test]
    #[should_panic]
    fn deletefile_error() {
        deletefile::deletefile("does_not_exist").unwrap();
    }

    #[test]
    fn deletefile_success() {
        let name = "delete_file_tests";
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(name).unwrap();

        deletefile::deletefile(name).unwrap();

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn fibonacci_error() {
        let result = fibonacci::fibonacci(0);
        assert_ne!(0, result.unwrap());
    }

    #[test]
    fn fibonacci_success() {
        let result = fibonacci::fibonacci(0);
        assert_eq!(0, result.unwrap());

        let result = fibonacci::fibonacci(1);
        assert_eq!(1, result.unwrap());

        let result = fibonacci::fibonacci(100);
        assert_eq!(354224848179261915075, result.unwrap())
    }

    #[test]
    #[should_panic]
    fn hash_error() {
        let result = hash::hash("Hello");
        assert_eq!(result, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn hash_success() {
        let result = hash::hash("hello");
        assert_eq!(result, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn random() {
        use crate::functions::random;

        let count = 5;
        let min = 10;
        let max = 200;

        let result = random::random(count, min, max);
        assert!(result.is_ok());

        let vector = result.unwrap();
        assert_eq!(count, vector.len());

        for number in vector {
            assert!(
                number >= min && number <= max,
                "{} is outside provided range: {} <= x <= {}",
                number, min, max
            );
        }
    }

    #[test]
    #[should_panic]
    fn random_minmax_error() {
        use crate::functions::random;

        let count = 5;
        let min = 10;
        let max = 9;

        let result = random::random(count, min, max);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic]
    fn random_count_error() {
        use crate::functions::random;

        let count = 5;
        let min = 10;
        let max = 11;

        let result = random::random(count, min, max);
        assert!(result.is_ok());

        let vector = result.unwrap();
        assert_ne!(count, vector.len());
    }

    #[test]
    fn reverse() {
        let result = reverse::reverse("HeLlO");
        assert_eq!("OlLeH", result);
        assert_ne!("olleh", result);
    }

    #[test]
    #[should_panic]
    fn reverse_case_error() {
        let result = reverse::reverse("HeLlO");
        assert_ne!("OlLeH", result);
    }

    #[test]
    fn timestamp() {
        let timestamp = timestamp::timestamp();
        let timestamp: Vec<&str> = timestamp.split(&['-', 'T', ':', '+']).collect();
        assert_eq!(8, timestamp.len());
    }

    #[test]
    fn toupper() {
        let result = toupper::toupper("hello world!");
        assert_eq!("HELLO WORLD!", result);
    }
}
