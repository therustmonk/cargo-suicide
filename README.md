## cargo-suicide

<aside class="warning">
WARNING! Use this program at your own risk. It can drop recent changes of your project.
</aside>

If you feel it's time to start living in a new way than this `cargo` subcommand is for you.

Install it:

```
cargo install suicide
```

Change a current working directory to your project, add changes to the code and run:

```
cargo suicide
```

If your new code contains any warning it will run `git reset --hard`.
If you had written shitcode before it will drop any changes everytime.

If the subcommand dropped your changes, but you feel happy because your editor
remains open than try `--god` mode:

```
cargo suicide --god
```

It will kill you editor as well.

Kill the shitcoder in you!

<aside class="notice">
But remember, at the time of writing this subcommand, I accidentally
killed `vim` with unsaved changes of an important project 😢
</aside>
