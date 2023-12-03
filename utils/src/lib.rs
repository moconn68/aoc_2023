use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Reades the lines of a [`BufRead`] into an [`Iterator`] of [`String`]s.
/// Under the hood this strips any [`Err`]s from the underlying iteration, returning only valid inputs.
///
/// * `reader` [`BufRead`] handle to the input source ([`File`], [`Cursor`](std::io::Cursor), etc.)
///
/// Returns an [`Iterator`] over the lines in the input buffered reader as [`String`]s.
pub fn read_to_lines(reader: impl BufRead) -> impl Iterator<Item = String> {
    reader.lines().flatten()
}

/// Reads an input file into an iterator over lines of text.
/// This is a convenience function for [`read_to_lines`] that builds the file handle for you.
/// Based on <https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach>.
///
/// Parameters:
/// * `file_path` - [`AsRef<Path>`] that represents the full path (including name) for the input file.
///
/// Returns an [`Iterator`] over the lines in the input buffered reader as [`String`]s.
/// Will return an [`std::io::Error`] if the file cannot be opened.
pub fn read_file_lines(
    file_path: impl AsRef<Path>,
) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let reader = BufReader::new(File::open(file_path)?);
    Ok(read_to_lines(reader))
}
