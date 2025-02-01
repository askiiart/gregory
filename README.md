# Gregory

This is Gregory. Gregory controls repos. Gregory keeps track of updating repos, trying to be simple and elegant, but enough.

## Documentation

Install gregory with `cargo install`:

```sh
cargo install --git https://github.com/askiiart/gregory
```

Gregory's config looks something like this:

```toml
max-jobs = 4
max-threads = 10

[packages]

  [packages.librewolf]

  dependencies = ["some-librewolf-dependency"]
  version_check = ["check-version --whenever-you-feel-like-it-please"]

    [packages.librewolf.compilation]
    revision = "2"
    threads = 6
    image = "docker.io/library/debian"
    commands = ["echo hi", "sleep 2.432", "echo helloooooooooo"]
    volumes = ["librewolf"]
```

For more details, look at the `./docs/`, and check out the rest of the [example config](./gregory.example.toml).

Once you've created your config, just run gregory with `gregory run` - that's it!

## TODO

- Add multithreading
- Add hook system

## Other stuff

- The formatting for the config file (`gregory.toml`) was heavily inspired by Drone's config.
- Why the name?
  - I was thinking to go with something dark and foreboding, since this is a program to control *everything* about many repos - it's the high command. But I couldn't think of anything and thought just naming it some lame random name instead would be way funnier. Hence, Gregory.
- Gregory is a program, so it uses it/its pronouns. It also doesn't mind whether you capitalize its name or not, "gregory" or "Gregory" are fine, you can even shorten it if you want.
- It's built for updating package repositories, but can be used to run pretty much anything. This isn't to say support won't be offered unless you're using it for a repo, but development will be focused on updating repos.
