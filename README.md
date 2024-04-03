## `sniptip`
Sniptip is a simple tool used to store some particular text (in my case: **code snippets**)
in your machine and be able to access it later using the tool.

## Usage

Run `sniptip help` for info on how to use the tool

Init (Run this first)
```bash
sniptip init
```

Save a snippet
```bash
sniptip add {name} {snippet}
```

Show a snippet
```bash
sniptip show {name}
```

List snippets saved
```bash
sniptip list
```

## Installation
Use the `install.sh` script to install in your machine or download
the binaries from the [releases](https://github.com/anfelo/sniptip/releases) list.

```bash
curl -LSfs https://anfelo.github.io/scripts/install.sh | \
    sh -s -- --git anfelo/sniptip
```

For more details about this installation script see install.sh -h
