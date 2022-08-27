pub mod cursor;
pub mod file;
pub mod tracer;
pub mod traverser;
pub mod utils;

pub use cursor::*;
pub use tracer::*;
pub use traverser::*;

#[cfg(test)]
mod tests {
    use crate::cursor::*;
    use crate::file::*;

    #[test]
    fn test_php() {
        let file = File::new("./test_php/test_class.php", tree_sitter_php::language()).unwrap();
        let mut cursor = Cursor::from_file(&file, STKind::Concrete);

        assert_eq!(cursor.goto_first_child(), true);
        assert_eq!(cursor.goto_first_child(), false);
        eprintln!("{}", cursor.kind());
        assert_eq!(cursor.goto_next_sibling(), true);
        assert_eq!(cursor.name(false), Some("Data".into()));
    }

    #[test]
    fn test_javascript() {
        let file = File::new(
            "./test_javascript/test_example.js",
            tree_sitter_javascript::language(),
        )
        .unwrap();
        let mut cursor = Cursor::from_file(&file, STKind::Concrete);

        assert_eq!(cursor.goto_first_child(), true);
        assert_eq!(cursor.goto_first_child(), true);
        assert_eq!(cursor.goto_first_child(), true);
        assert_eq!(cursor.goto_first_child(), true);
        assert_eq!(cursor.kind(), "<");
    }
}
