# Lines Of Code Counter
Counts LOC in source files ( cpp,h,rs,py,c, ... ) by recursing into given path and also reports percentages of the LOC per file extension.

# Build & Run instructions : 

1. Install Rust: https://rustup.rs
2. Clone or unzip project
3. cd project
4. cargo build
5. cargo run -- [path]

# Example Run :
cargo run -- ../../

```
Discovering source files in : ../../
File traversal done. 16 suitable source files found.
Total lines of code : 21112
cpp        =         54 ( % 0.2557787 )
rs         =      21058 ( % 99.74422 )
```
