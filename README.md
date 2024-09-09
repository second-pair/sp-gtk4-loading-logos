#  Second-Pair's GTK4 Loading Logos Library
Nice simple library that draws loading-logos into a GTK4 widget.  This is implementing as a library the following project:

	https://github.com/second-pair/rust-gtk4-loading-logos

##  C Interoperability
I will be making cross-compilation for C a priority and will be what I work on first.

I will be following my buildsystem at work, creating an archive file, currently for the following architectures:
 -  x86_64-unknown-linux-gnu -> sp-gtk4-loading-logos.linux.a
 -  aarch64-unknown-linux-gnu -> sp-gtk4-loading-logos.arm.a
 -  x86_64-pc-windows-gnu -> sp-gtk4-loading-logos.win.a
