# Rust Ownership: Preventing Use-After-Free Errors

This is my project for COMP 211 Checkoff 3. We have been working with pointers, `malloc`, and `free` in C, so I wanted to see how Rust deals with a similar memory error.

I focused on **use-after-free**: when a program tries to read or write memory after it has already been freed. I tested an example in C and compared it with a similar example in Rust.

## What I tried

### 1. Use-after-free in C

File: [use_after_free.c](use_after_free.c)

The C program allocates space for an integer, stores `42`, and sets another pointer, `q`, equal to `p`. Both pointers refer to the same allocation. After `free(p)`, the code tries to print `*q`, even though that memory is no longer valid to access.

I tested this in Compiler Explorer using GCC with AddressSanitizer enabled. **The C code compiled, but AddressSanitizer reported a `heap-use-after-free` error when the program ran.**

Screenshot: [C AddressSanitizer result](c-address-sanitizer.png)

The program has *undefined behavior*. Without a tool like AddressSanitizer, it might appear to work, crash, or do something else. Printing `42` would not mean the code was safe.

### 2. The equivalent mistake in Rust

File: [rejected.rs](rejected.rs)

In the Rust example, `Box::new(42)` puts an integer on the heap, and `value` owns that allocation. `reference` borrows the integer using `&*value`. The program then tries to call `drop(value)` before using `reference`.

I tested this in Rust Playground. **It did not compile.** The compiler returned error **E0505**, saying it could not move out of `value` because it was borrowed.

Screenshot: [Rust compiler error E0505](rust-e0505.png)

This was the part that surprised me. I initially expected the Rust program to compile and possibly crash, as a similar program could in C. Instead, Rust rejected the code before running it.

### 3. Fixing the Rust program

File: [fixed.rs](fixed.rs)

The fix is to print through the reference **before** calling `drop(value)`. The borrow is no longer needed after its last use, so the owner can safely be dropped.

The corrected program prints `42` when compiled and run.

## How to reproduce the examples

### C (Compiler Explorer)

1. Open [Compiler Explorer](https://godbolt.org/) and set the language to **C**.
2. Copy the contents of [use_after_free.c](use_after_free.c) into the editor.
3. Select an x86-64 GCC compiler and use these compiler options:

```
-g -O0 -fsanitize=address
```

4. Enable **Execute the code** in the compiler settings.
5. Open the output panel. You should see an AddressSanitizer `heap-use-after-free` error.

You can also compile with a local GCC that supports AddressSanitizer:

```bash
gcc -g -O0 -fsanitize=address -fno-omit-frame-pointer use_after_free.c -o use_after_free
./use_after_free
```

This example intentionally contains a memory error and is meant to be run with a memory diagnostic tool.

### Rust (Rust Playground)

1. Open [Rust Playground](https://play.rust-lang.org/).
2. Paste the contents of [rejected.rs](rejected.rs) and select **Run**. You should see compiler error E0505 rather than normal program output.
3. Replace it with [fixed.rs](fixed.rs) and select **Run** again. This version should print `42`.

## What I learned

- In C, it's possible to have two pointers to the same allocation and accidentally keep using one after calling `free`.
- Rust
