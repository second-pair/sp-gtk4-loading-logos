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


##  Credit
A lot of the base information for the cross-language compilation came from this webpage:
 -  http://blog.asleson.org/2021/02/23/how-to-writing-a-c-shared-library-in-rust/
