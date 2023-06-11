# rustc-sat

This code turns a SAT problem in DIMACS CNF format into Rust code that compiles
iff the SAT problem is unsatisfiable, thus proving that compiling Rust is
NP-hard. See [the blog post](https://niedzejkob.p4.team/rust-np/) for details.

Using the rustc SAT solver a single solution 3x3 sudoku has been solved in 23 seconds.
