# Textbook Opener CLI

The Textbook Opener is a command-line tool designed to help students and educators quickly access textbooks and solutions manuals from a specified directory based on the class name. This tool navigates to a designated textbook path, lists all relevant files, and opens either the textbook or the solutions manual according to user specifications.

## Features

* Navigate to Class Directory: Automatically finds and navigates to the textbook directory for a specified class.
* Search and List Textbook Files: Searches all files in the class directory and lists them if multiple options are available.
* Open Specific Files: Depending on the user’s choice, opens the textbook or the solutions manual.
* Interactive Selection: If multiple files are found, the user can specify which file to open.
* Error Handling: Provides clear error messages if files or directories are missing.

## Usage

To use the Textbook Opener, you’ll need to provide the class name and specify whether you want to open the textbook or the solutions manual. The tool also supports opening files in a browser for an enriched viewing experience.

## Basic Command Structure
```textbooks [options] <class>```

## Options

* -h: Display help message and usage information.
* -s: Open the solutions manual instead of the textbook.
* -b: Open the textbook or solutions manual in the browser.

## Examples

* Open Textbook for a Class: textbooks ece250
* Open Solutions Manual for a Class: textbooks -s ece250
* Open in Browser: textbooks -b ece250

## Error Messages

If issues arise, the tool provides informative error messages guiding the user to resolve common problems such as missing directories or files.

## Configuration

The Config struct allows customization of the command execution, supporting class specification, solutions manual selection, and browser-based viewing.

## Error Handling

The tool uses a custom TextbookError type.

## Development

This tool is written in Rust, offering robust performance and safety features. It uses conditional compilation for cross-platform compatibility and leverages Rust’s powerful error handling capabilities.
