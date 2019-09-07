for each language, pipe the output into `head -1`. if it displays a nonsense error message you win.

| language   | build step         | run cmd      | error on stdout closing                                                                                                                                                                       |
|------------|--------------------|--------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| C          | `clang main.c`     | `./a.out`    | none                                                                                                                                                                                          |
| C++        | `clang++ main.cpp` | `./a.out`    | none                                                                                                                                                                                          |
| Rust [1]   | `rustc main.rs`    | `./main`     | `thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', src/libstd/io/stdio.rs:743:9 note: Run with RUST_BACKTRACE=1 environment variable to display a backtrace.` |
| Go         | `go build main.go` | `./main`     | none                                                                                                                                                                                          |
| JavaScript | none               | `./main.js`  | none                                                                                                                                                                                          |
| Python     | none               | `./main.py`  | none                                                                                                                                                                                          |
| Ruby       | none               | `./main.rb`  | none                                                                                                                                                                                          |
| Racket     | none               | `./main.rkt` | none                                                                                                                                                                                          |
| GForth     | none               | `./main.4th` | none                                                                                                                                                                                          |

[1] Run `rustc main_safe.rs; ./main_safe | head -1` for the version that checks for broken pipe errors
