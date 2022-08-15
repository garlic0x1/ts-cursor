use crate::cursor::*;
use crate::file::*;
use crate::traverser::*;

pub struct Dumper<'a> {
    files: Vec<&'a File>,
}

impl<'a> Dumper<'a> {
    /// create a dumper from a vec of files
    pub fn new(files: Vec<&'a File>) -> Self {
        Self { files }
    }

    /// associated function to dump individual cursors
    pub fn dump_cursor(cursor: Cursor<'a>, concrete: bool) -> String {
        let mut string = String::new();
        let mut depth = 0;

        let trav = Traversal::from_cursor(&cursor, concrete);
        for cur in trav {
            match cur {
                Order::Enter(cur) => {
                    let indent = "  ".repeat(depth);
                    string.push_str(&format!("{}Kind: {}\n", indent, cur.kind()));
                    if cur.kind() == "name" {
                        string.push_str(&format!("{}Name: {}\n", indent, cur.to_str()));
                    }
                    if let Some(field) = cur.raw_cursor().field_name() {
                        string.push_str(&format!("{}Field: {}\n", indent, field));
                    }

                    depth += 1;
                }
                Order::Leave(_) => {
                    depth -= 1;
                }
            }
        }

        string
    }

    pub fn dump(&self, concrete: bool) -> String {
        println!("dumping");
        let mut string = String::new();

        for file in self.files.iter() {
            string.push_str(&Dumper::dump_cursor(file.cursor(true), concrete));
        }

        string
    }
}
