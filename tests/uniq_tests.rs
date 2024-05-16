#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use uniq::UNIQ;

    pub const FILE_PATH: PathBuf = PathBuf::from("test_files/test.txt");
    #[test]
    fn test_print_lines() {
        let uniq = UNIQ::new(FILE_PATH);
        uniq.print_lines();
    }

    #[test]
    fn test_print_uniq() {
        let uniq = UNIQ::new(FILE_PATH);
        uniq.print_uniq();
    }

    #[test]
    fn test_print_duplicates() {
        let uniq = UNIQ::new(FILE_PATH);
        uniq.print_duplicates();
    }

    #[test]
    fn test_count_unique_lines() {
        let uniq = UNIQ::new(FILE_PATH);
        uniq.count_unique_lines();
    }

    #[test]
    fn test_print_counters() {
        let uniq = UNIQ::new(FILE_PATH);
        uniq.print_counters(); 
    }
}