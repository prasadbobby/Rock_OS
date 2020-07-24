# Rock OS

OS made using Rust

## Setting up the Work Environment

You have to install the rust nightly for the time being as the few dependencies are not yet stabe.<br>
Install the rust package manager: <br>
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
<br>
Install the source code of cargo to get the access of the `core` and `compiler_builtins` crates: <br>
`rustup component add rust-src` 
<br>
Install nightly version: <br>
`rustup update nightly --force`

## Running

For running you should have [QEMU](https://www.qemu.org/). <br>
After installing, run: <br>
`qemu-system-x86_64 -drive format=raw,file=target/x86_64-rock_os/debug/bootimage-rock_os.bin`<br>
or
<br>
`cargo run`

*OR* 

### Caution
**Only experienced programmers should use this, if wrongly done you might get your PC formatted.**<br>
You can boot it in a USB and run in a real machine.
Run:
`dd if=target/x86_64-blog_os/debug/bootimage-rock_os.bin of=/dev/sdX && sync` <br>

Here in `sdX` X is the number of the usb port you get by running: <br>
`diskutil list` 