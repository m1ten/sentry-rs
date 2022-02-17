# sentry

yet another multi-purpose discord bot

| :warning: WARNING                                        |
|:--------------------------------------------------------:|
| This project is incomplete and may not work as expected. |

## Installation

Download the latest binary from [releases](https://github.com/m1ten/sentry-rs/releases)

```sh
# POSIX: Give execution permission to sentry and run
$ chmod +x sentry && ./sentry

# Windows: Run the exe
$ .\sentry.exe
```
### Building from source 

1. Install dependencies
   1. [Rust >=1.57](https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html) using [`rustup`](https://www.rust-lang.org/tools/install)
   ```sh
   $ rustup toolchain install stable
   ```

2. Clone the [source](https://github.com/m1ten/sentry-rs) using [`git`](https://git-scm.com/)
    ```sh
    $ git clone https://github.com/m1ten/sentry-rs.git
    $ cd sentry
    ```
3. Build and run using [`cargo`](https://doc.rust-lang.org/stable/cargo/)
    ```sh
    $ cargo +stable run --release
    ```

## Contributors

- [miten](https://github.com/m1ten/) - Current Maintainer
- [Gabry](https://github.com/Gabryx64/) - Major Contributor

## License

sentry is licensed under [Apache-2.0](./LICENSE).