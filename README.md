# index-gen

a cli tool for generating index.html file recursively for a directory

# Usage

```
Generate index.html file recursively for a directory

Usage: index-gen [OPTIONS]

Options:
  -d, --dir <DIR>    Root dir to generate, default is current dir [default: .]
  -n, --name <NAME>  The index file name [default: index.html]
  -f, --force        Override if the index file already exists
  -a, --all          Do not ignore entries starting with `.`
      --remove       Recursively remove all index file
      --json         Do not generate file, only print JSON
      --string       Do not generate file, only print String
  -h, --help         Print help
  -V, --version      Print version
```

# Todo

-   [x] display file size
-   [ ] custom css
-   [ ] sort file and dirs
