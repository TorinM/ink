# Ink

Ink is a gap buffered text editor written in Rust that is meant to be ultra performant

## Structure

Each line of text is represented as a dynamic gap-buffered array providing ammortized O(1) time
complexity for its operation.

During initialization, we calculate the buffer as being the length of the screen, with the screen
being a 2D array of text. This supports monospaced data. This way, the gap will only be resized
if the user has a line of text that is longer than their screen (which is unlikely.

