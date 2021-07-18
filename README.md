# nettl

nettl is a small unitilty combining Windows netstat.exe and tasklist.exe to provide netstat-like functionality with actual process names instead of PIDs.

## Usage

```
NETTL [-t] [-u] [-l]

  -t            Display TCP connections.
  -u            Display UDP connections.
  -l            Display LISTENING connections.
```

### Compiling from source code

1. clone the project
1. install Rust: [instructions from the official site](https://www.rust-lang.org/learn/get-started)
1. go to the project folder and run: `cargo build --release`
1. fetch compiled binary from the target\release folder

## License

nettl is licensed under the BSD 3-Clause "New" or "Revised" License.
