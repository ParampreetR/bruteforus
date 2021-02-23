# bruteforus - bruteforce URLs

`bruteforus` is a simple tool written in rust for bruteforus URLs and find some hidden files and folders on webserver.


## Installation

```
git clone https://github.com/ParampreetR/bruteforus.git
cd bruteforus
make install 
```

## Usage

```
bruteforus 0.1.0
Used to bruteforce urls.

USAGE:
    bruteforcer <url> --wordlist <wordlist>

ARGS:
    <url>
            Target URL

OPTIONS:
    -w, --wordlist <wordlist>
            Wordlist to bruteforce with URL

    -t, --threads <threads>
            Number of threads to perform requests

        --wait <wait>
            MilliSecs to wait after each request

    -h, --help
            Prints help information

    -V, --version
            Prints version information
```

## License

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/rust-lang/libc/blob/master/LICENSE-MIT))


