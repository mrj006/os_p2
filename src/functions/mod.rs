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
//pub mod loadtest; ya no se va a usar este
pub mod help;
pub mod status;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn file() {
        let name = "test";
        let content = "Hello World!\n";
        let repeat: u64 = 5;

        createfile::createfile(name, content, repeat).unwrap();

        let file = fs::OpenOptions::new()
            .read(true)
            .open("test")
            .unwrap();

        assert_eq!(content.len() as u64 * repeat, file.metadata().unwrap().len());

        deletefile::deletefile(name).unwrap();
    }

    #[test]
    #[should_panic]
    fn createfile_error() {
        let name = "test";
        let content = "Hello World!\n";
        let repeat: u64 = 5;

        createfile::createfile(name, content, repeat).unwrap();
        createfile::createfile(name, content, repeat).unwrap();
    }

    #[test]
    #[should_panic]
    fn deletefile_error() {
        deletefile::deletefile("does_not_exist").unwrap();
    }

    #[test]
    fn fibonacci() {
        let result = fibonacci::fibonacci(0);
        assert_eq!(0, result);

        let result = fibonacci::fibonacci(1);
        assert_eq!(1, result);

        let result = fibonacci::fibonacci(100);
        assert_eq!(354224848179261915075, result)
    }

    #[test]
    fn hash() {
        let result = hash::hash("hello");
        assert_eq!(result, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");

        let result = hash::hash("Hello");
        assert_ne!(result, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn random() {
        let count = 5;
        let min = 10;
        let max = 200;
        let vector = random::random(count, min, max);
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
    fn reverse() {
        let result = reverse::reverse("HeLlO");
        assert_eq!("OlLeH", result);
        assert_ne!("olleh", result);
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
