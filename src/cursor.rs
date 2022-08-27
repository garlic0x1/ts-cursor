use crate::file::*;
use crate::tracer::Trace;
use crate::traverser::*;
use std::hash::{Hash, Hasher};
use tree_sitter::*;
use STKind::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum STKind {
    Abstract,
    Concrete,
}

#[derive(Clone)]
pub struct Cursor<'a> {
    cursor: TreeCursor<'a>,
    file: &'a File,
    pub stkind: STKind,
}

/// empty trait means it uses partial_eq
impl<'a> Eq for Cursor<'a> {}

impl<'a> PartialEq for Cursor<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.file.name() == other.file.name() && self.cursor.node().id() == other.cursor.node().id()
    }
}

impl<'a> Hash for Cursor<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.name().hash(state);
        self.cursor.node().id().hash(state);
    }
}

impl<'a> std::fmt::Debug for Cursor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.kind().to_string();
        if let Some(name) = self.name(true) {
            s.push(' ');
            s.push_str(&name);
        }

        write!(f, "{}", s)
    }
}

impl<'a> Cursor<'a> {
    /// wrap a cursor with its associated file
    pub fn from_cursor(cursor: TreeCursor<'a>, file: &'a File, stkind: STKind) -> Self {
        Self {
            cursor,
            file,
            stkind,
        }
    }

    /// create a cursor at the root node of the file
    pub fn from_file(file: &'a File, stkind: STKind) -> Self {
        Self {
            cursor: file.raw_cursor(),
            file,
            stkind,
        }
    }

    /// traverse inside current node
    pub fn traverse(&self, stkind: STKind) -> Traversal {
        Traversal::from_cursor(&self, stkind)
    }

    /// traverse without crawling into break nodes
    pub fn traverse_block(&self, breaks: Vec<&'a str>, stkind: STKind) -> Traversal {
        Traversal::from_block(&self, breaks, stkind)
    }

    /// trace up the syntax tree
    pub fn trace(&self) -> Trace {
        Trace::new(self.clone())
    }

    pub fn filename(&self) -> String {
        self.file.name()
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn kind(&self) -> &str {
        self.cursor.node().kind()
    }

    pub fn field(&self) -> Option<&str> {
        self.cursor.field_name()
    }

    /// get the tree_cursor object for complete control
    pub fn raw_cursor(&self) -> TreeCursor<'a> {
        self.cursor.clone()
    }

    // navigation

    pub fn goto_parent(&mut self) -> bool {
        self.cursor.goto_parent()
    }

    pub fn goto_first_child(&mut self) -> bool {
        let mut ret = self.cursor.goto_first_child();
        if self.stkind == Abstract {
            while !self.raw_cursor().node().is_named() {
                ret = self.cursor.goto_next_sibling();
            }
        }
        ret
    }

    pub fn goto_next_sibling(&mut self) -> bool {
        let mut ret = self.cursor.goto_next_sibling();
        if self.stkind == Abstract {
            while !self.raw_cursor().node().is_named() {
                ret = self.cursor.goto_next_sibling();
            }
        }
        ret
    }

    // go to nth child (indexing starts at zero)
    pub fn goto_child(&mut self, index: usize) -> bool {
        if !self.goto_first_child() {
            return false;
        }

        let c = (0..)
            .take_while(|i| *i <= index && self.goto_next_sibling())
            .count();

        if c != index {
            self.goto_parent();
        }

        c == index
    }

    /// move cursor to field, true if success
    pub fn goto_field(&mut self, field: &str) -> bool {
        self.goto_first_child();
        while self.field() != Some(field) {
            if !self.goto_next_sibling() {
                self.goto_parent();
                return false;
            }
        }
        true
    }

    /// try to find the name of current node
    /// if true, crawl depth first until name
    /// if false, only search one layer
    pub fn name(&self, deep: bool) -> Option<String> {
        // if name, return
        if self.kind() == "name" {
            return Some(self.to_string());
        }

        // then look for a name child
        {
            let mut cur = self.clone();
            cur.goto_first_child();
            while cur.goto_next_sibling() {
                if cur.kind() == "name" {
                    return Some(cur.to_string());
                }
            }
        }

        // if deep crawl depth first
        // this finds weird things like object names that dont have a direct name child
        if deep {
            for motion in self.traverse(Abstract) {
                if let Order::Enter(cur) = motion {
                    if cur.kind() == "name" {
                        return Some(cur.to_string());
                    }
                }
            }
        }

        None
    }

    /// get which child index we are in
    pub fn get_index(&self) -> usize {
        let node_id = self.cursor.node().id();
        let mut index = 0;
        let mut cursor = self.cursor.clone();

        // handle cases where we are checking root node
        if !cursor.goto_parent() {
            return 0;
        }

        cursor.goto_first_child();

        while cursor.node().id() != node_id {
            if cursor.node().is_named() {
                index += 1;
            }
            cursor.goto_next_sibling();
        }

        index
    }

    /// get the source code of the current node
    pub fn to_str(&self) -> &str {
        let node = self.cursor.node();
        let slice = &self.file.get_source()[node.byte_range()];
        slice
    }

    /// get the source code of the current node
    pub fn to_string(&self) -> String {
        let node = self.cursor.node();
        let slice = &self.file.get_source()[node.byte_range()];
        slice.to_string()
    }
}
