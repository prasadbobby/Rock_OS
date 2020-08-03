# Rock OS

OS made using Rust<br>

<!-- ![Build](https://github.com/HarshitRuwali/Rock_OS/workflows/Build/badge.svg?branch=master) -->
<!-- Removing the Build badge due the bug in the nightly version of rust. -->

## Setting up the Work Environment

You have to install the rust nightly for the time being as the few dependencies are not yet stabe.<br>
Install the rust package manager: <br>
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
<br>
Install the source code of cargo to get the access of the `core` and `compiler_builtins` crates: <br>
`rustup component add rust-src` 
<br>
Install nightly version: <br>
`rustup override add nightly`

## Running Rock OS

### Running Virtually

For running you should have [QEMU](https://www.qemu.org/). <br>
After installing, run: <br>
`cargo run`<br>
That would install the dependencies/crates and open the OS in the QEMU virtual machine.

### Running in the Real Machine 

#### Caution 

**Only experienced programmers should use this, if wrongly done you might get your system formatted.**<br>
You can boot it in a USB and run in a real machine.
Run:
`dd if=target/x86_64-blog_os/debug/bootimage-rock_os.bin of=/dev/sdX && sync` <br>

Here in `sdX` X is the number of the usb device you get by running: <br>
`diskutil list` 

## Have a suggestion?
Create an issue and mention it. 

## Want to contribute?
Fork the repo, make the changes and push a pull request. 

## Ref
Awesome guide by @phil-opp: [Writing an OS in Rust](https://os.phil-opp.com/).

## Contact Me
You may message me on [Telegram](https://t.me/harshitruwali) or shoot out a [mail](https://mail.google.com/mail/?view=cm&fs=1&to=ruwaliharshit@gmail.com).
