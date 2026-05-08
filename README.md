# Steam Workshop Analyzer

<div align="center">
  <img src="assets/logo/swa.png" alt="SWA logo" width="220"/>
</div>

## Description

SWA is an open source CLI tool that lets you do a quick analysis on the disk space your Steam workshop mods are taking, it also lets you get a Steam name given its app id (e.g. 108600 -> Project Zomboid).

It works for both Windows and Linux. In order to analyze the disk space, you must have mods installed.

## Demo

![SWA demo](assets/demo/swa_demo.gif)

## Usage

Follow these instructions if you want to run or build the program from the source code:

Make sure to have cargo installed, to check if you already have it execute this command on the terminal.
```bash
cargo --version
```
If you see something like the following message: 

> `cargo` is not recognized as an internal or external command, operable program or batch file.

it means you don't have it installed, you can install it from here: https://doc.rust-lang.org/cargo/getting-started/installation.html

The message may vary, this one is a Windows specific message.

To run it, go to the project directory and execute this command.
```bash
cargo run
```

If you want to build it and get the binary file, you can execute this command.
```bash
cargo build --release
```

If you want to run the program easily from anywhere in your terminal, execute this command.
```bash
cargo install --path .
```
