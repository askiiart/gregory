# CLI Arguments

Add `-h` or `--help` to any subcommand to view its help.

## Run (`run`)

```txt
Usage: gregory run [OPTIONS]
```

**Options:**

- `-c`, `--config`: Path to the config file; default: `gregory.toml`
<!-- - `-d`, `--daemonize`: Whether to daemonize the program - not yet supported -->

## Generate shell completions `gen-completion`

```txt
Usage: gregory gen-completion [OPTIONS] <COMMAND>
```

**Commands:**

- bash
- zsh
- fish
- elvish
- powershell

**Options:**

- `-b`, `--binary-name`: The name of the binary; default: `gregory`
