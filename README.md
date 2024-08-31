# Felys Future

Immature thoughts about Felys and build some unstable components.

## Future

I have been thinking what kind of problem I want Felys to solve that it not solved my any available language. Let me first quickly talk about my thoughts about some well-known programming language.

### Those I used

- Python: the first language I learnt and mastered; do like this language because of the syntax, effectiveness, and itself as a C project; slow and a bit too dynamic
- JavaScript: really easy to get started, but the type system sucks
- TypeScript: kind of solved the typing issue in JS, but made things a bit too verbose to me
- Rust: love every aspect of this language, except for the lifetime checker (for now, probably also async when I get there in the future)
- C: simple, controllable, and flexible, I enjoy doing both LeetCode problems and projects using it
- C++: way too flexible, would never learn it if university does not ask for; hate it

### Those I have never used

- Java: like the language design (JVM), but it is way too verbose
- Lua: also like the design as a modern interpreted language

For any others, I either does not have a chance to see it or it does not attract me at all.

### About Felys

In short, the future Felys will probably be something like a interpreted Rust which does not have borrow checker and lifetime checker. The memory management will be using garage collection.

Some future features:

- static typing: catch tons of stupid error at compile time
- virtual machine: do garage collection and JIT compilation
- compiler: optimize the code and generate bytecode

It will be a less verbose Java, simpler Rust, and more strict Python.

## Plan

Unlike the full-stack version of current Felys, the future version will have a front-end and back-end separated architecture. The former will stay in Rust, but the latter will move to C for more flexibility and better performance. Then it will bind to Rust and Python again for the interface like the CLI and REPL.

### Front-end

I am familiar with PEG, which is powerful enough, so I will stick with it.

### Back-end

Still a newbie.

### Progress

Currently reading the dragon book to get some academic knowledge in building programming language; definitely going to read some books related to JIT compiler and garbage collection.
