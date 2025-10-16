# Ink

Ink is a gap buffered text editor written in Rust that is meant to be "ASAM" (As Minimal As Possible) while providing "Vim Like" editing tools.

This project is currently in a very-early stage, see [contributions](#contributions) to see steps to apply for changes.

## Usage

Ink lets you focus on a single file at a time to maximize the experience. You can open a file using

```bash
ink <file_name>
```
- `<file_name>` should be either the name of an existing file or a new file to create. Use relative path.

### Operators

At the core of Ink are operating modes, which provide different operations:

1. Operator (`O`): The default mode which allows for saving the file and safely exit.
2. Editor (`E`): Editing mode, which allows for text insertions. Use arrow keys to navigate the file.

### Switching

To switch an editor at any time, use the `Ctrl+[KEY]` pattern. For example, to switch to "Editor" mode, use `Ctrl+e`

## Road Map

- Allow for cursor movement with arrow keys

## Dependencies

- [termion](https://github.com/redox-os/termion/tree/master)

## Contributions

[See contributing guidelines](/CONTRIBUTING.md#contributing-guildlines)
