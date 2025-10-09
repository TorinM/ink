# Ink

Ink is a gap buffered text editor written in Rust that is meant to be ASAM (As Minimal As Possible) while providing "Vim Like" editing tools. 

This project is currently in a very-early stage, see [contributions]() to see steps to apply for changes.

## Usage

Ink lets you focus on a single file at a time to maximize the experience. You can open a file using

```bash
ink <file_name>
```

### Operators

Ink has multiple operating modes (Vim-like) for you to modify your file with

1. Operator (`O`): The default mode which allows for saving the file and safely exiting the editor
2. Editor (`E`): Editing mode, which allows for text insertions. Use arrow keys to navigate the file.

## Road Map

- Allow for multi-line editing


## Known Issues

- Backspace does not remove the text itself, rather just moves the gap pointers


## Contributions

To submit a feature request or bug, please submit a pull request to this README.md file with additions to the "Road Map" or "Known Issues" sections.

For larger updates, please submit a pull request and I will review.

