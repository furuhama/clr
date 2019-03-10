## clr

Colorize csv file

### What's this?

Improve a readability of raw csv file from terminal command line.

### Build

Cargo build at first, and you will get `clr` executable file.

```
$ cargo build --release
```

Set PATH to a generated executable file.

### Usage

You just run `clr` command with target CSV file path.

```
$ clr [target CSV file]
```

Or, pass a column-separated data directly as a STDIN.

```
$ cat [target CSV file] | clr
```

### Support

- UTF-8
- SHIFT-JIS

### Screenshot

![sample01](https://github.com/furuhama/clr/blob/master/img/sample01.png)
