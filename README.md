# rust-dietician

Measure how chubby your rust program is.

Specifically, answer the question "why is my rust binary so big?",
which pops up from time to time on the `#rust-beginners` IRC channel.
This is intended to slurp in a rust binary and print out a summary of
what's taking up how much space.

# Example

The easiest way to run it is with `cargo`:

```
cd rust-dietician
cargo run
```

By default, it prints out stats for its own binary, looking something like this:

```
Section size breakdown:
Code                1487 Kb
Data                 687 Kb
Debug               2686 Kb
Metadata             547 Kb
Other                  2 Kb

Symbol size breakdown:
C functions          210 Kb
C data                15 Kb
Consts                11 Kb
Strings               14 Kb
Refs                  28 Kb
Exception tables       1 Kb
Panic locations       16 Kb
Rust functions      1239 Kb
Misc. Rust data       10 Kb
Sytem info             1 Kb
VTables                4 Kb

Size of all symbol info: 1541 Kb
Total size: 5406 Kb
```

You can add `-h` to get help:

```
cargo run -- -h
```

Or `-v` or `-vv` to get more detailed info on what it's actually finding (which gets spammy)

```
cargo run -- -vv
```

# What it's actually measuring

An ELF executable file contains one or more **sections**, which are just chunks of space with a
particular size (and flags such as read-only, execute-only, etc which we don't care about right now).
Each section can contain pretty much whatever it wants to; in addition to the traditional code and data
there are debug symbols, compiler version notes, relocation tables for DLL's, jump tables, strings, and
whatever else the compiler might need to put somewhere.

An ELF file may also contain **symbols**, which are just pointers into a section that refer to some
arbitrary data.  They *usually* tell you the location of useful things such as functions or public
variables, but can be all sorts of other things.  It's just a name attached to something.

## Sections

This is a high-level breakdown of what your Rust program contains.  "Code" is almost always actually
code.  "Data" is data, BSS (uninitialized data), strings, or a few other minor things.  "Metadata" is
stuff like DLL symbol references, offset tables and other stuff that gets nestled around your program so
the OS can do all the things that it needs to do to run things.  "Debug" info is debugging info.

## Symbols

What most of these actually are is a bit of a guess, but if it looks like a Rust value, it's classified
as such, otherwise it's assumed to be a C value.  

## But why is my program so fat?

Short answer is usually standard library and debugging info.  Build in release mode if you want, but
really, don't sweat it.  You're usually happier with stack traces than with a binary being 40% smaller.
If you aren't, you know it and know how to slim it down.

## Why don't the total symbol sizes match the total section sizes?

Not everything in a binary has a symbol referring to it.  Symbols are only necessary if something outside
the program needs to find a particular value.  Things like private variables and functions don't
necessarily have entries in the symbol table (though with Rust they often do, for debugging purposes).

# Weaknesses

* It can only decode ELF files.  So if you're on a platform that
  doesn't use ELF (Windows, Mac, maybe some BSD's) it will probably
  still take apart ELF files, but not native executables.
* The Rust ABI and symbol mangling rules are not, as far as I know,
  stable.  So while it currently works with Rust 1.11, there's no
  guarentee that it will work with earlier or subsequent versions.
* No promises that the classifications are actually correct.  For all
  I know the symbol `const3263` in the `.data` segment is actually a
  function pointer.


# Building

Should be able to just check it out and build it with `cargo build`.


# Todo

* Document section and symbol classes more
* Break down rust symbol names so it can give you stats by module
* Try to give stats for function specializations
* Try to break it
* Make it callable via `cargo dietician`
