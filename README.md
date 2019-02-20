Command to be used with lighthouse (https://github.com/emgram769/lighthouse).

Searches a folder for matches and executes a command on matching path, suitable for launching
terminals, editors etc

Can use a prefix for selecting command.

## Usage

```
lighthouse-command <command> [<prefix>|<command>...] --folder=<folder>


A `command` is a string that will  be used as a template for the selected folder. Use `{}` for placement of selected folder name.
Ex. `"atom {}"`

A prefix lets you add extra command accesible by typing a charachter and a space either before search term or after.
Ex `"c|code {}"` adds a command that can be triggered by typing something like "c foobar" or "foobar c" into lighthouse.

`--folder` specifies the folder to search.

Example, with `lighthouserc` pointing at `lighthouse-command`

```sh
lighthouse -- "alacritty --working-directory ~/projects/{}" "a|atom {}" "c|code {}" "g|xdg-open https://github.com/davidlgj/{}" --folder=~/projects | sh
```

