#  Second-Pair's GTK4 Loading Logos Library
Nice simple library that draws loading-logos into a GTK4 widget.  This is implementing as a library the following project:
 -  https://github.com/second-pair/rust-gtk4-loading-logos

##  C Interoperability
This library will be cross-compiled into C and provide archive files in the top-level directory.  Simply add one of these to your compilation as if it were a C-file and include 'sp-gtk4-loading-logos.h'.

###  Supported Targets

| Build Target | Triple                    | Archive Name                  |
| :----------- | :------------------------ | :---------------------------- |
| Linux        | x86_64-unknown-linux-gnu  | sp-gtk4-loading-logos.linux.a |
| Win          | aarch64-unknown-linux-gnu | sp-gtk4-loading-logos.arm.a   |
| Arm          | x86_64-pc-windows-gnu     | sp-gtk4-loading-logos.win.a   |


##  Archive Sizes
This was interesting.  Compiling with the 'release' profile with 'debug = true' yielded some mammoth archive files.  Adding a few more overrides got this down very quickly.  So I ran a few more tests.

Details about these settings can be found here:
 -  https://doc.rust-lang.org/rustc/codegen-options/index.html

I didn't try stripping symbols, as we need these kept in the archive for the implementing programme to decide what to do with them.

1.  lto = "fat"
2.  lto = "thin"
3.  codegen-units = 1
4.  panic = 'abort'
5.  #! [no_std]
6.  opt-level = 'z'
7.  opt-level = 's'
8.  opt-level = '0'
9.  opt-level = '1'
10.  opt-level = '2'
11.  opt-level = '3'
12.  codegen-units = 2

From doing some looking around / tapping my own knowledge:
 -  opt-level - basically it looks like Release uses -O2 by default.  We can save a bit of space with 's' and 'z', but not when combined with LTO etc.
 -  Link-Time Optimisation has been a gimme since 2020; assuming the time-hit isn't too great.
 -  Thin LTO is supposedly faster than Fat LTO and keeps most of the optimisation benefits.
 -  'codegen-units' is basically "Job Count" - reducing this reduces parallelism while compiling, but allows that one thread to make much better optimisations.
 -  Panic-handling we could get by without, but it doesn't save much (<0.6MB after LTO).
 -  We need Rust-std and not using it saves basically nothing.

###  Sizes

Units are in MB.

| Vanilla release | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | A | B | C | 1346 | 13 | 23 | 236 | 237 | 2C |
| :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- | :- |
| 115 | 8.1 | 9.1 | 84 | 112 | 115 | 98 | 93 | 189 | 172 | 115 | 115 | 96 | 7.5 | 7.7 | 7.9 | 7.7 | 7.7 | 8.2 |

###  Timings
Basically:

| 13 | 23 | 2C | 13 Incremental | 23 Incremental |
| :- | :- | :- | :- | :- |
| 43.933 | 42.532 | 36.408 | 4.757 | 1.989 |

My summary from all of this is that we should use Fat LTO & 1 Codegen Unit; until compilation becomes a big drag, at which point we should switch to Thin LTO & 2 Codegen Units.  Time will tell whether we should just dive straight into the latter, or if the former will be worthwhile.

##  Credit

A lot of the base information for the cross-language compilation came from this webpage:
 -  http://blog.asleson.org/2021/02/23/how-to-writing-a-c-shared-library-in-rust/

The information for optimising the build sizes came from a few places.  Off the top, we've got:
 -  https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge
 -  https://doc.rust-lang.org/rustc/codegen-options/index.html
