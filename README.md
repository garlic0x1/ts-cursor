# ts-cursor
Wrapper for tree-sitter trees and cursors to simplify taint analysis.  
Cursors can traverse and trace up the syntax tree.  
This was designed for PHP but should work for most languages.  

# examples
`examples/dumper.rs` is a command line tool for dumping syntax trees. (javscript and php only)  
Build with
```
  $ cargo build --examples dumper
  $ ./target/debug/examples/dumper --concrete example.php php # concrete php tree
  $ ./target/debug/examples/dumper example.js js # abstract js tree
```

# see also
https://github.com/skmendez/tree-sitter-traversal provides traversers that can handle one direction at a time,
which is fine if you dont need to push/pop a context stack.
