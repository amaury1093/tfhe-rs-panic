# TFHE-rs Panic Reproduction

## Summary

When calling `tfhe::generate_keys`, I get a panic at the end of the program execution (not during the function call). The panic is:

```
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
unsafe precondition(s) violated: ptr::replace requires that the pointer argument is aligned and non-null
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread caused non-unwinding panic. aborting.
[1]    26769 abort      cargo run
```

## Reproduction

```bash
cargo run
```

## Expected

Program exits with 0 code.

## Actual

```
➜  tfhe-rs-panic git:(main) ✗ cargo run
   Compiling tfhe-rs-panic v0.1.0 (/Users/amaury/Workspace/inco/tfhe-rs-panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/tfhe-rs-panic`
Done.
thread '<unnamed>' panicked at library/core/src/panicking.rs:220:5:
unsafe precondition(s) violated: ptr::replace requires that the pointer argument is aligned and non-null
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread caused non-unwinding panic. aborting.
[1]    26769 abort      cargo run
➜  tfhe-rs-panic git:(main) ✗ echo $?
134
```
