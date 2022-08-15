use crate::cursor::*;
use crate::file::*;

pub struct Traversal<'a> {
    cursor: Cursor<'a>,
    start: Cursor<'a>,
    last: Option<Cursor<'a>>,
    blocks: Vec<&'a str>,
    concrete: bool,
    visited: bool,
    end: bool,
}

impl<'a> Traversal<'a> {
    /// abstract traversal (only named nodes)
    pub fn new(cursor: &Cursor<'a>) -> Self {
        Self {
            start: cursor.clone(),
            last: None,
            cursor: cursor.clone(),
            visited: false,
            concrete: false,
            end: false,
            blocks: Vec::new(),
        }
    }

    pub fn from_file(file: &'a File) -> Self {
        Self {
            start: file.cursor(),
            last: None,
            cursor: file.cursor(),
            visited: false,
            concrete: false,
            end: false,
            blocks: Vec::new(),
        }
    }

    /// abstract block traversal, does not crawl into specified node kinds
    pub fn new_block(cursor: &Cursor<'a>, blocks: Vec<&'a str>) -> Self {
        Self {
            start: cursor.clone(),
            last: None,
            cursor: cursor.clone(),
            visited: false,
            concrete: false,
            end: false,
            blocks,
        }
    }

    /// concrete traversal (all nodes)
    pub fn new_concrete(cursor: &Cursor<'a>) -> Self {
        Self {
            start: cursor.clone(),
            last: None,
            cursor: cursor.clone(),
            visited: false,
            concrete: true,
            end: false,
            blocks: Vec::new(),
        }
    }

    /// skip over this node
    pub fn pass(&mut self) {
        //eprintln!("passing {}", self.last.clone().unwrap().to_str());
        // if the one the user wants to skip isnt the first, go back to there, else end
        if let Some(cur) = &self.last {
            self.cursor = cur.clone();
        } else {
            self.end = true;
        }

        // switch to visited so we leave it
        self.visited = true;
    }

    /// next item in traversal
    pub fn step(&mut self) -> Option<Order<'a>> {
        let last = self.cursor.clone();
        self.last = Some(last.clone());

        if self.end {
            return None;
        }

        if self.visited {
            // break when we have completely visited start
            if last == self.start {
                self.end = true;
            }

            // if visited go to next sibling or parent and leave visited node
            if self.cursor.goto_next_sibling() {
                // we havent visited this yet, break out of leave loop
                self.visited = false;
                return Some(Order::Leave(last));
            } else if self.cursor.goto_parent() {
                return Some(Order::Leave(last));
            } else {
                // break if we are at the root node
                self.end = true;
                return Some(Order::Leave(last));
            }
        } else {
            // if not visited, keep entering child nodes
            if self.cursor.goto_first_child() {
                return Some(Order::Enter(last));
            } else {
                // we are at a leaf, turn around
                self.visited = true;
                return Some(Order::Enter(self.cursor.clone()));
            }
        }
    }
}

/// ordering wrapper for cursor
#[derive(Clone)]
pub enum Order<'a> {
    Enter(Cursor<'a>),
    Leave(Cursor<'a>),
}

/// preorder and postorder together, for pushing and popping to context stack
impl<'a> Iterator for Traversal<'a> {
    type Item = Order<'a>;

    /// get the next step in iteration
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.step() {
            match &item {
                Order::Enter(cur) => {
                    if self.concrete || cur.raw_cursor().node().is_named() {
                        if self.blocks.contains(&cur.kind()) {
                            self.pass();
                        }
                        return Some(item);
                    }
                }
                Order::Leave(cur) => {
                    if self.concrete || cur.raw_cursor().node().is_named() {
                        return Some(item);
                    }
                }
            }
        }
        None
    }
}
