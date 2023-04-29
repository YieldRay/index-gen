# index-gen

a cli tool for generating index.html file recursively for a directory

# Usage

```
Usage: index-gen [OPTIONS]

Options:
  -d, --dir <DIR>      Root dir to generate, default is current dir [default: .]
  -n, --name <NAME>    The index file PREFIX name, will automatically use correct extension name [default: index]
  -f, --force          Override if the index file already exists
      --inject <HTML>  Inject some html to <head> of the index html
  -a, --all            Do not ignore entries starting with `.`
      --remove         Recursively remove all index file
      --html           Generate html
      --json           Generate JSON
      --txt            Generate Txt
  -h, --help           Print help
  -V, --version        Print version
```

```sh
# inject some custom style to <head>
index-gen --inject "<link rel=\"stylesheet\" href=\"https://unpkg.com/landsoul@latest/dist/landsoul.css\" />"
```

# Todo

-   [x] display file size
-   [x] custom head
-   [ ] sort file and dirs
