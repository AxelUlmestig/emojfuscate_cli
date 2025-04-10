
# Emojfuscate

This CLI is a wrapper around the seminal [Emojfuscate library](https://github.com/AxelUlmestig/emojfuscate/). It converts data into emoji and back again.

## Installation

#### From Source
```bash
make install-executable
```

## Example usage

#### Simplest Use Case
```bash
$ emojfuscate encode -l "carcinisation"
💴🪶🧈💦🎯🌸🎡👀🦔🐚🕔📑🎴💵
$
$ emojfuscate decode -l 💴🪶🧈💦🎯🌸🎡👀🦔🐚🕔📑🎴💵
carcinisation
```
#### Stdin
Data can also be passed from stdin by passing a dash (`-`) instead of text
```bash
$ cat /bin/ls | emojfuscate encode - > my_ls.emoji
$
$ head --bytes 40 my_ls.emoji 
💴🥯😰☀🍗😆🤩😀😀😀
$
$ cat my_ls.emoji | emojfuscate decode - > my_ls
$
$ chmod +x my_ls
$
$ ./my_ls ~/Documents/
my-favourite-crabs.csv epstein-flight-logs.txt
```

In the above example we can see that emojfuscate has a [data compression ratio](https://en.wikipedia.org/wiki/Data_compression_ratio) of ~0.31
```bash
$ ls -l my_ls*
-rwxrwxr-x 1 axel axel 142312 apr 10 21:03 my_ls
-rw-rw-r-- 1 axel axel 452413 apr 10 21:01 my_ls.emoji
```
142312 / 452413 = 0.314562136808624 

#### More Specific Data Types

Data types can be specified with the `--data-type` (or `-d`) flag:
```bash
$ emojfuscate encode -l --data-type uuid 1c3e7189-b881-4def-bad4-ddabdeadbeef
😺🧮😠💐🍔🧒📸🛥🪅🛺🪮🥀📜📣
$
$ emojfuscate decode -l --data-type uuid 😺🧮😠💐🍔🧒📸🛥🪅🛺🪮🥀📜📣
1c3e7189-b881-4def-bad4-ddabdeadbeef
```

```bash
$ emojfuscate encode -l --data-type hexadecimal abc123
💴🚕😗📑🤜💵
$
$ emojfuscate decode -l --data-type hexadecimal 💴🚕😗📑🤜💵
abc123
```

The default `--data-type` is `text`.
