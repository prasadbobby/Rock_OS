#Rock OS

## Using

You have to install the rust nightly for the time being as the few dependencies are not yet stabe.<br>
Install the rust package manager: <br>
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
<br>
Install the source code of cargo to get the access of the `core` and `compiler_builtins` crates: <br>
`rustup component add rust-src` 
<br>
Install nightly version: <br>
`rustup update nightly --force`