use super::traverser::*;
use crate::cursor::{Cursor, STKind};
use anyhow::Result;
use tree_sitter::*;

pub struct File {
    name: String,
    source: String,
    tree: Tree,
}

impl File {
    pub fn new(name: &str, language: Language) -> Result<Self> {
        let source = std::fs::read_to_string(name)?;
        let mut parser = Parser::new();
        parser.set_language(language)?;
        let tree = parser.parse(&source, None).unwrap();
        return Ok(File::from_tree(name, tree, source));
    }

    pub fn from_tree(name: &str, tree: Tree, source: String) -> Self {
        Self {
            name: name.to_string(),
            source,
            tree,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn raw_cursor(&self) -> TreeCursor {
        self.tree.walk()
    }

    pub fn cursor(&self, stkind: STKind) -> Cursor {
        Cursor::from_cursor(self.raw_cursor(), self, stkind)
    }

    pub fn traverse(&self, stkind: STKind) -> Traversal {
        Traversal::from_file(self, stkind)
    }

    // I dont think I want this

    // pub fn from_url(url: &str, language: Language) -> Result<Self> {
    //     let client = reqwest::blocking::Client::builder()
    //         .user_agent("g4r1cI's super sweet scanner")
    //         .timeout(Duration::from_secs(5))
    //         .build()?;
    //     let source = client.get(url).send()?.text()?;
    //     let mut parser = Parser::new();
    //     parser.set_language(language)?;
    //     let tree = parser.parse(&source, None).unwrap();
    //     Ok(File::from_tree(url, tree, source))
    // }
}
