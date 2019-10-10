# `sheth` CLI

The `sheth` CLI provides a command line interface to the tooling which supports
the execution environment.

## Packager

The packager is used to generate random transaction packages (transactions +
multiproof). It's usage is defined as:

```
USAGE:
    client package [FLAGS] [OPTIONS] <accounts> <transactions>

FLAGS:
    -h, --help       Prints help information
        --scout      When set, the output will be in the format of a Scout YAML file
    -V, --version    Prints version information

OPTIONS:
    -d, --height <height>    defines the height of sparse state structure [default: 256]

ARGS:
    <accounts>        number of accounts that will be represented in the proof
    <transactions>    number of transactions to be generated
```

## Client

The client is an interactive tool which maintains the full state of the
execution environment (e.g. all the accounts) and can process a few commands
which lets users submit transactions and monitor balances.

```
USAGE:
    client start [OPTIONS] <accounts>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --height <height>    defines the height of sparse state structure [default: 256]

ARGS:
    <accounts>    number of accounts that will be represented in the proof
```

