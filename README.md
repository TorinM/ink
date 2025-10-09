# Ink

Ink is a gap buffered text editor written in Rust that is meant to be "ASAM" (As Minimal As Possible) while providing "Vim Like" editing tools.

This project is currently in a very-early stage, see [contributions](#contributions) to see steps to apply for changes.

## Usage

Ink lets you focus on a single file at a time to maximize the experience. You can open a file using

```bash
ink <file_name>
```

### Operators

Ink has multiple operating modes (Vim-like) for you to modify your file with

1. Operator (`O`): The default mode which allows for saving the file and safely exit
2. Editor (`E`): Editing mode, which allows for text insertions. Use arrow keys to navigate the file.

## Road Map

- Allow for cursor movement with arrow keys

## Known Issues

- Adding newline to the buffer does not reset the cursor to the first column

## Dependencies

- [termion](https://github.com/redox-os/termion/tree/master)

## Contributions

[See contributing guidelines](/CONTRIBUTING.md#contributing-guildlines)

