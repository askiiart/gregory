# Config Reference

- Default config location: `gregory.toml`
- **Example**: see [`gregory.example.toml`](/gregory.example.toml)

It's recommended to edit the config file using [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) for VS Code with the following options:

```json
    "evenBetterToml.formatter.indentTables": true,
    "evenBetterToml.formatter.indentString": "  "
```

Note: This primarily uses LibreWolf and Fedora as examples of packages and distros. Also note that rather than separating by what distro, you can instead use those field to define which repo.

## Top-level config

- `log-level` (integer): Log level `0`-`3` (error, warning, info, or debug)
  - Default: 1 - warning
- `max-threads` (float): The maximum number of threads to be used
  - **See also**: [`threads`](#job-config)
  - Default is CPU's threads - 2
- `max-jobs` (integer): The maximum number of jobs to be run at once
  - Default is 1
- `data-dir` (string): The path to put data for job logs and stuff
  - **Temporary**, will be removed once SQL database support is added

**Multithreading notes (IMPORTANT)**: Gregory will first run compilation jobs, then packaging jobs for whatever programs are done, then run the `update-repo` for whichever distros are finished. For this reason, the distro names listed under `packaging` and `update-repo` *must* match.

**Multithreading is not implemented yet**

## Job config

- `id` (string): An ID to identify the job, such as the compilation of a program **(highly recommended)**
  - Default is `-1` for unassigned
  - If you just want to run stuff, you don't need this, but it's *highly* recommended as it allows you to filter your logs.
- `revision` (string): A revision id for the job, such as a version number for a compilation script
  - Default is `1`
- `threads` (float): The maximum number of vCPUs/threads to dedicate to a job; this can be a fractional number
  - Set this as less than or equal to the max number of threads the thing you're running will use
  - See `--cpus` in the [`podman run` docs](https://docs.podman.io/en/latest/markdown/podman-run.1.html#cpus)
  - *Root may be required for this argument*
  - If not specified, it will fall back to `max-threads`
- `image` (string): The Docker image to run the job in **(required)**
- `commands` (array): The commands to run **(required)**
  - Note than you can use single-quote strings instead for string literals - see [TOML docs](https://github.com/toml-lang/toml/blob/main/toml.md#string) for details
- `volumes` (array): Names of volumes as defined in [`volumes` (top level)](#volumes)
- `privileged` (bool): Whether the job's container should be privileged
- `shell` (string): The shell to run the commands in
  - Default: `/bin/sh`

Note: `id` and `revision` are *not* for the package version, they are for 

## Packages (`packages`)

Example:

```toml
[packages]

  [packages.librewolf]

    [packages.librewolf.compilation]
    id = "1"
    revision = "2"
    threads = 8
    image = "docker.io/library/debian"
    commands = ["echo hi", "echo helloooooooooo"]
    volumes = ["librewolf"]

    [packages.librewolf.packaging.fedora]
    threads = 8
    image = "docker.io/library/fedora"
    commands = [
      "echo did you ever hear the tragedy of darth plageuis the wise?",
      "echo it\\'s not a story the jedi would tell you",
    ]
    volumes = ["librewolf"]
```

### Compilation (optional)

Defines the compilation of a program, if applicable. Stuff like Python scripts can skip this.

It's defined in this format:

```toml
[packages]

  [packages.librewolf]

    [packages.librewolf.compilation]
    id = "1"
    revision = "2"
    threads = 8
    image = "docker.io/library/debian"
    commands = ["echo hi", "echo helloooooooooo"]
    volumes = ["librewolf"]
```

### Packaging

Defines the packaging of a program into stuff like `.deb` or `.rpm` files.

Example:

```toml
[packages.librewolf.packaging.fedora]
threads = 8
image = "docker.io/library/fedora"
commands = [
  "echo did you ever hear the tragedy of darth plageuis the wise?",
  "echo it\\'s not a story the jedi would tell you",
]
volumes = ["librewolf"]

[packages.librewolf.packaging.debian]
threads = 4
image = "docker.io/library/debian"
commands = [
  "echo hiiiiiii"
]
```

Replace `distro-name` with the name of a distro, like `fedora` or `debian`

## Update repo (`update-repo`)

Defines how to update a repo.

Example:

```toml
[update-repo]

  [update-repo.fedora]
  threads = 4
  image = 'docker.io/library/fedora'
  commands = ["echo hai"]
  volumes = ["librewolf"]
```

## Volumes

Lists a volume in Docker/Podman's volume format, to be used in [job configs](#job-config)

```toml
[volumes]
librewolf = "./local/path:/path-in-container"
```
