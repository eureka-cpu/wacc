# Chapter 1

I bought this book because I wanted to be able to write my own compilers, so hopefully this will give me just enough insight to do that.
I don't expect it to be the golden bullet that will teach me everything I need to know, but I'm excited to write my first compiler.

## Setup

Just went through the setup, unfortunately it turns out I can't use an aarch64-linux system, which is what I was on when I started.
Maybe once I get things rolling I'll go back and write a way to do checks using nix to cross compile things or run it in a virtual machine,
or even better just make the compiler compatible with aarch64 instructions.

I switched to an x86_64-linux laptop after pushing to a new repo, cloned it and ran `nix develop`, checked against the book's setup checker and I'm good to go.

## Problems

- `return_2` doesn't execute

On the bottom of page 6, there is this snippet
```
gcc return_2.s -o return_2
./return_2
echo $?
```
I had to `chmod +x return_2` and run it again because the output was 0 instead of 2 (probably it didn't execute?). It returned 2 after adding executable permissions.
