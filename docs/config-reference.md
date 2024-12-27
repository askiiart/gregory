# Config Reference

- Default config location: `gregory.yml`
- Example: see [`gregory.example.yml`](/gregory.example.yml)

Note: This primarily uses LibreWolf and Fedora as examples of packages and distros. Also note that rather than separating by what distro, you can instead use those field to define which repo.

## Top-level config

- `log-level` (integer): Log level `0`-`3` (error, warning, info, or debug)
  - Default: 1 - warning
- `max-threads` (integer): The maximum number of threads to be used
  - **See also**: [`threads`](#job-config)
  - Default is CPU's threads - 2
- `max-jobs` (integer): The maximum number of jobs to be run at once
  - Default is 1

**Multithreading notes (IMPORTANT)**: Gregory will first run compilation jobs, then packaging jobs for whatever programs are done, then run the `update-repo` for whichever distros are finished. For this reason, the distro names listed under `packaging` and `update-repo` *must* match.

**Multithreading/multiple jobs is not implemented yet**

## Job config

- `threads` (integer): The maximum number of vCPUs/threads to dedicate to a job; this can be a fractional number
  - Set this as less than or equal to the max number of threads the thing you're running will use
  - See `--cpus` in the [`podman run` docs](https://docs.podman.io/en/latest/markdown/podman-run.1.html#cpus)
  - *Root may be required for this argument*
  - If not specified, it will fall back to `max-threads`
- `image` (string): The Docker image to run the job in *(required)*
- `commands` (sequence): The commands to run *(required)*
  - TODO: Add command file/bash script instead
- `volumes` (sequence): Names of volumes as defined in [`volumes` (top level)](#volumes)
- `privileged` (bool): Whether the job's container should be privileged

## Packages (`packages`)

Example:

```yml
packages:
  librewolf:
    compilation:
      image: 'debian'
      commands:
        - './mach build'
    
    packaging:
      fedora:
        image: 'lesbi-oops-i-mean/debian'
        commands:
          - './lesbiab package thingy'
```

### Compilation (optional)

Defines the compilation of a program, if applicable. Stuff like Python scripts can skip this.

It's defined in this format:

```yml
packages:
  pkgname:
    compilation:
      image: 'fedora'
      commands:
        - 'echo hi'

  other-package:
    compilation:
      job-details-go-here:
```

### Packaging

Defines the packaging of a program into stuff like `.deb` or `.rpm` files.

Example:

```yml
packages:
  pkgname:
    packaging:
      distro-name:
        image: 'fedora'
        commands:
          - 'echo hi'
  
  other-package:
    packaging:
      distro-name:
        job-details-go-here:
```

Replace `distro-name` with the name of a distro, like `fedora` or `debian`

## Update repo (`update-repo`)

Defines how to update a repo.

Example:

```yml
update-repo:
  distro-name:
    image: 'fedora'
    command:
      - 'echo hi'
```

## Volumes

Lists a volume in Docker/Podman's volume format, to be used in [job configs](#job-config)

```yml
volumes:
  librewolf: './local/path:/path/in/container'
```
