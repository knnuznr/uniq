#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use uniq::UNIQ;

    #[test]
    fn test_print_lines() {
        let file_path = PathBuf::from("test_files/test.txt");
        let uniq = UNIQ::new(file_path);
        uniq.print_lines();
    }

    #[test]
    fn test_print_uniq() {
        let file_path = PathBuf::from("test_files/test.txt");
        let uniq = UNIQ::new(file_path);
        uniq.print_uniq();
    }

    #[test]
    fn test_print_duplicates() {
        let file_path = PathBuf::from("test_files/test.txt");
        let uniq = UNIQ::new(file_path);
        uniq.print_duplicates();
    }

    #[test]
    fn test_count_unique_lines() {
        let file_path = PathBuf::from("test_files/test.txt");
        let uniq = UNIQ::new(file_path);
        uniq.count_unique_lines();
    }

    #[test]
    fn test_print_counters() {
        let file_path = PathBuf::from("test_files/test.txt");
        let uniq = UNIQ::new(file_path);
        uniq.print_counters(); 
    }
}