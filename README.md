# Rust Ownership: Preventing Use-After-Free Errors

This is my project for COMP 211 Checkoff 3. We have been working with pointers, `malloc`, and `free` in C, so I wanted to see how Rust deals with a similar memory error.

I focused on **use-after-free**: when a program tries to read or write memory after it has already been freed. I tested an example in C and compared it with a similar example in Rust.

## What I tried

### 1. Use-after-free in C

File: [`c/use_after_free.c`](c/use_after_free.c)

The C program allocates space for an integer, stores `42`, and sets another pointer, `q`, equal to `p`. Both pointers refer to the same allocation. After `free(p)`, the code tries to print `*q`, even though that memory is no longer valid to access.

I tested this in Compiler Explorer using GCC with AddressSanitizer enabled. **The C code compiled, but AddressSanitizer reported a `heap-use-after-free` error when the program ran.** The screenshot is in [`screenshots/c-address-sanitizer.png`](screenshots/c-address-sanitizer.png).

The program has *undefined behavior*. Without a tool like AddressSanitizer, it might appear to work, crash, or do something else. Printing `42` would not mean the code was safe.

### 2. The equivalent mistake in Rust

File: [`rust/rejected.rs`](rust/rejected.rs)

In the Rust example, `Box::new(42)` puts an integer on the heap, and `value` owns that allocation. `reference` borrows the integer using `&*value`. The program then tries to call `drop(value)` before using `reference`.

I tested this in Rust Playground. **It did not compile.** The compiler returned error **E0505**, saying it could not move out of `value` because it was borrowed. The screenshot is in [`screenshots/rust-e0505.png`](screenshots/rust-e0505.png).

This was the part that surprised me. I initially expected the Rust program to compile and possibly crash, as a similar program could in C. Instead, Rust rejected the code before running it.

### 3. Fixing the Rust program

File: [`rust/fixed.rs`](rust/fixed.rs)

The fix is to print through the reference **before** calling `drop(value)`. The borrow is no longer needed after its last use, so the owner can safely be dropped. The corrected program should print `42`.

## How to reproduce the examples

### C (Compiler Explorer)

1. Open [Compiler Explorer](https://godbolt.org/) and set the language to **C**.
2. Copy the contents of [`c/use_after_free.c`](c/use_after_free.c) into the editor.
3. Select an x86-64 GCC compiler and use these compiler options:

   ```text
   -g -O0 -fsanitize=address
   ```

4. Enable **Execute the code** in the compiler settings.
5. Open the output panel. You should see an AddressSanitizer `heap-use-after-free` error.

You can also compile with a local GCC that supports AddressSanitizer:

```bash
gcc -g -O0 -fsanitize=address -fno-omit-frame-pointer c/use_after_free.c -o use_after_free
./use_after_free
```

This example is **intentionally invalid C code** and is meant to be run with a memory diagnostic tool.

### Rust (Rust Playground)

1. Open [Rust Playground](https://play.rust-lang.org/).
2. Paste [`rust/rejected.rs`](rust/rejected.rs) and select **Run**. You should see compiler error E0505, not normal program output.
3. Replace it with [`rust/fixed.rs`](rust/fixed.rs) and select **Run** again. This version should print `42`.

## What I learned

- In C, it's possible to have two pointers to the same allocation and accidentally keep using one after calling `free`.
- Rust's ownership and borrowing rules can stop this particular mistake at compile time. Here, the error occurs because the program tries to destroy the owner while a borrowed reference will still be used.
- A borrow can end after its last use. That's why the corrected Rust program can call `drop(value)` after printing.
- This does **not** mean every Rust program is automatically free of all bugs. The strongest memory-safety guarantees apply to safe Rust; incorrectly written `unsafe` Rust can still cause memory errors.

## Presentation and sources

The slides are in [`slides/`](slides/), and the experiment screenshots are in [`screenshots/`](screenshots/). Links to relevant documentation and tools are in [`REFERENCES.md`](REFERENCES.md).
