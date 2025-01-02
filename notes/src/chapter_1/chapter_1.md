# Chapter 1

I bought this book because I wanted to be able to write my own compilers, so hopefully this will give me just enough insight to do that.
I don't expect it to be the golden bullet that will teach me everything I need to know, but I'm excited to write my first compiler.

## Setup

Just went through the setup, unfortunately it turns out I can't use an aarch64-linux system, which is what I was on when I started.
Maybe once I get things rolling I'll go back and write a way to do checks using nix to cross compile things or run it in a virtual machine,
or even better just make the compiler compatible with aarch64 instructions.

I switched to an x86_64-linux laptop after pushing to a new repo, cloned it and ran `nix develop`, checked against the book's setup checker and I'm good to go.

## Lexer

Ok, so I finished my lexer according to the spec in the book, in Rust. It said to trim whitespace and use a match expression combined with regex to collect tokens.
I don't really like that at all if I'm being honest, so I made a small compromise. To retain the spans I just use a mutable counter to track the current position,
and the `regex` crate combined with generative macros for pattern matching. I still think this looks pretty bad, but I'm not sure how to make it better using regex.
I hope the book offers other solutions, but if not I'll have a look through some other lexers and try to come up with a better solution. This one just feels sort of
overly verbose, and wasteful of cycles. I do feel proud of what's there though, because matching on regex isn't supported natively in rust, and I was able to find a
way around it using macros. I also feel a bit disappointed in the way I collected the tokens, I was really hoping I could use a `Token` trait object to avoid using
extra enums.

## Problems

- `return_2` doesn't execute

On the bottom of page 6, there is this snippet
```
gcc return_2.s -o return_2
./return_2
echo $?
```
I had to `chmod +x return_2` and run it again because the output was 0 instead of 2 (probably it didn't execute?). It returned 2 after adding executable permissions.

## Questions

Q. I'm sort of confused while going through the compiler driver portion on pages 7 and 8.
I got through most of it, but the part which describes the `--lex`, `--parse` and `--codegen` options
is sort of vague, in that I'm not exactly sure how the options will be used _in the code_ at this point
but the pages leading up to this made it seem as if it's important I know this already.
For example, sure, `--lex` is a `bool` that allows the `test_compiler` script to tell the compiler driver
to lex only, then stop. But lex what? Where is this option being passed, to my yet-to-be-written compiler or gcc?

