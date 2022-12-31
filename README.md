# `pipetee`

[<img alt="crates.io" src="https://img.shields.io/crates/v/pipetee.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/pipetee)

A simple, fast, no-dependencies UNIX utility to print the contents of stdin to
the terminal *and* forward them to stdout at the same time.

## Install

- Requires `rust 1.65+`.
- Only works on a \*nix system.

Via cargo:

```sh
cargo install pipetee
```

Via repo:

```sh
git clone github.com/mark-i-m/pipetee
cd pipetee
cargo install --path .
```

## Example usage:

```sh
# output from pt will interleave with output from
# sed at the granularity of the buffer

$ yes 'yes' | pt | sed 's/yes/no/g'
no
no
no
no
no
..
no
yes
yes
yes
yes
yes
..
yes
no
no
no
no
no
..
no
^C

$ yes 'yes' | ./target/release/pt -b 3 | sed 's/yes/no/g'
yeno
s
yno
es
no
no
yesno
no
no

yes
yno
es
no
no
yesno
no
^C
```

```sh
# look at intermediate results if the final output takes a long time.

$ cat numbers.txt | ./computation-with-incremental-results.py | pt | sort -n | tee output.txt
# unsorted
0.0030632556724745847
0.0018044960059851569
0.001587923806107082
0.0029520906739906577
0.006344797296449427
0.1628663298523446
0.10106032701405257
0.028920997961789503
0.027188567279582024
0.02568497042514556
# after a while... sorted
0.001587923806107082
0.0018044960059851569
0.0029520906739906577
0.0030632556724745847
0.006344797296449427
0.02568497042514556
0.027188567279582024
0.028920997961789503
0.10106032701405257
0.1628663298523446
```

## How does it work?

It reads from stdin and writes to both stdout and /dev/tty. Your shell will
redirect stdout to whatever the next pipe is, but /dev/tty (what stdout
defaults to on most shells) will remain untouched.
