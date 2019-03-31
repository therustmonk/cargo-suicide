## cargo-suicide

If you feel it's time to start living in a new way than this `cargo` subcommand is for you.

### Disclaimer

> **WARNING!** Use this program at your own risk. It can drop recent changes of your project.

### Installation

Install the tool using this command:

```
cargo install cargo-suicide
```

### Running

Move to your project's directory, add changes to the code and run:

```
cargo suicide
```

If your new code contains any warning it will run `git reset --hard`.
If you had written shitcode before it will drop any changes everytime.

### God Mode

If the subcommand dropped your changes, but you feel happy because your editor
remains open than try `--god` mode. It will kill you editor as well:

```
cargo suicide --god
```

![Gif with a Cat](https://media.giphy.com/media/uzglgIsyY1Cgg/giphy.gif)
