# rust-dietician

Measure how chubby your rust program is.

Specifically, answer the question "why is my rust binary so big?",
which pops up from time to time on the `#rust-beginners` IRC channel.
This is intended to slurp in a rust binary and print out a summary of
what's taking up how much space.

# Example

# What it's actually measuring

## Sections

## Symbols

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

* Document section and symbol classes
* Have command line args (simple ones at least)
* Break down rust symbol names so it can give you stats by module
* Try to give stats for function specializations
* Try to break it
