# cpl

Custom Programmer Launcher

## Configuration

The config is located at `~/.config/cpl`. In cpl's .config folder there should be another folder called `confs` where all the `.toml` configs should be located.

An example config file should look like:

- file.toml

```toml
[[example]]

[[example.applications]]
name = "file_search"
command = "ls"
args = "-a"
```

Running `cpl file.example` will launch the `ls -a` command in another window.
