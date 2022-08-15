use super::traverser::*;
use super::{cursor::*, traverser::Order};

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Resolved<'a> {
    Function { cursor: Cursor<'a> },
    Root { cursor: Cursor<'a> },
}

impl<'a> Resolved<'a> {
    pub fn new_function(cursor: Cursor<'a>) -> Self {
        Self::Function { cursor }
    }

    pub fn new_root(cursor: Cursor<'a>) -> Self {
        Self::Root { cursor }
    }

    pub fn cursor(&self) -> Cursor<'a> {
        match self {
            Resolved::Function { cursor } => cursor.clone(),
            Resolved::Root { cursor } => cursor.clone(),
        }
    }

    /// returns vec of resolved parameter names
    /// empty if not function variant
    pub fn parameters(&self) -> Vec<Cursor<'a>> {
        let mut v = Vec::new();

        match self {
            Resolved::Function { cursor } => {
                let mut cursor = cursor.clone();
                if !cursor.goto_field("parameters") {
                    return v;
                }
                let mut traversal = Traversal::new(&cursor);
                while let Some(motion) = traversal.next() {
                    if let Order::Enter(cur) = motion {
                        if cur.kind() == "variable_name" {
                            v.push(cur.clone());
                        }
                    }
                }

                v
            }
            // return empty if not function
            _ => v,
        }
    }

    /// for debugging
    pub fn dump_parameters(&self) -> String {
        let mut string = String::new();
        for cur in self.parameters().iter() {
            string.push_str(&cur.name().unwrap());
            string.push(' ');
        }
        string
    }

    pub fn name(&self) -> String {
        self.cursor().name().unwrap()
    }
}
