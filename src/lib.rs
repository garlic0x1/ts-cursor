pub mod cursor;
pub mod file;
pub mod tracer;
pub mod traverser;

#[cfg(test)]
mod tests {
    use crate::cursor::*;
    use crate::file::*;
    // use crate::tracer::*;
    // use crate::traverser::*;

    #[test]
    fn test_php() {
        let file = File::new("./test_php/test_class.php", tree_sitter_php::language()).unwrap();
        let mut cursor = Cursor::from_file(&file, true);

        assert_eq!(cursor.goto_first_child(), true);
        assert_eq!(cursor.goto_first_child(), false);
        eprintln!("{}", cursor.kind());
        assert_eq!(cursor.goto_next_sibling(), true);
        assert_eq!(cursor.name(false), Some("Data".into()));
    }

    #[test]
    fn test_javascript() {}
}
