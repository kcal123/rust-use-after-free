# Rust Ownership: Preventing Use-After-Free Errors

This project is for COMP 211 Checkoff 3. We've been learning about pointers and memory management in C, so I decided to look into how Rust, another programming language handles some of the same issues.

The main focus was use-after-free errors, which happen when a program tries to access memory after it's already been freed. I wanted to compare what happens when this mistake is made in C versus Rust.

## Testing use-after-free in C

I started with a short C program that allocates memory for an integer and stores the value 42. Then I created a second pointer to the same memory.

After calling `free(p)`, the program tries to access that memory again through `q`. Since the allocation has already been freed, this is undefined behavior.

The code is in [use_after_free.c](use_after_free.c).

I originally tried running the code through VS Code, but GCC wasn't available on my computer. Instead, I used Compiler Explorer with AddressSanitizer enabled.

The code compiled, but AddressSanitizer detected a `heap-use-after-free` error when it ran. Without AddressSanitizer, the program might have appeared to work, even though the memory access was invalid.

[C AddressSanitizer screenshot](c-address-sanitizer.png)

## Trying the same thing in Rust

Next, I tried a similar example in Rust using `Box::new(42)` to allocate an integer on the heap.

The variable `value` owns the Box, while `reference` borrows the integer stored inside it. The code then tries to destroy the Box using `drop(value)` before the reference has finished being used.

At first, I thought the program might compile and then crash, similar to what could happen in C. Instead, Rust refused to compile it.

The compiler gave error E0505 because the program was trying to drop a value that was still borrowed.

[Rust code that doesn't compile](rejected.rs)

[Rust compiler error screenshot](rust-e0505.png)

## Fixing the Rust code

The fix was fairly simple. Instead of dropping the Box before using the reference, I moved the `println!` statement before `drop(value)`.

This allows the reference to be used while the memory is still valid. Once the reference is no longer needed, the Box can safely be dropped.

The corrected program compiles and prints `42`.

[Corrected Rust code](fixed.rs)

## Running the examples

**C**

1. Open [Compiler Explorer](https://godbolt.org/) and select C.
2. Copy the code from [use_after_free.c](use_after_free.c).
3. Choose a GCC compiler and enter `-g -O0 -fsanitize=address` in the compiler options.
4. Enable **Execute the code**.
5. The output should show a `heap-use-after-free` error.

If GCC is installed locally with AddressSanitizer support, the program can also be compiled with:

`gcc -g -O0 -fsanitize=address -fno-omit-frame-pointer use_after_free.c -o use_after_free`

Then run `./use_after_free`.

**Rust**

1. Open [Rust Playground](https://play.rust-lang.org/).
2. Paste the code from [rejected.rs](rejected.rs) and click Run.
3. The compiler should return error E0505.
4. Replace it with [fixed.rs](fixed.rs) and run it again. The corrected version prints `42`.

## What I learned

The biggest difference I noticed was when each language detected the problem.

In C, the program compiled even though it contained a use-after-free error. AddressSanitizer caught it when the program ran.

In Rust, the ownership and borrowing rules prevented the program from compiling in the first place. The compiler recognized that a reference was still needed after the owner would have been destroyed.

Rust doesn't prevent every possible memory error, particularly when unsafe Rust is used, but its ownership rules prevented the mistake in this example.

## Presentation and references

[COMP 211 Presentation](Rust%20Use-After-Free%20-%20Kyle%20Caligan%20%284%29.pptx)

Additional documentation and resources are listed in [REFERENCES.md](REFERENCES.md).
