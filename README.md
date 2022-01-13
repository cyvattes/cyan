# Cyan Text Abstraction Tool
### CK Vattes (# 001237324)

## Clone the project

```
$ git clone https://github.com/cyvattes/cyan.git
$ cd cyan
```

## Install Rust

Unix
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
```
Windows
```
Download & run:
https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
```

## Run the project

```
$ cargo run cyan_api
```

## Usage

- Install Rust, clone, and run. The application runs at [localhost:51440/](http://localhost:51440/).
- Enter text to be summarized. Please note that the abstractor works best on objective/declarative text (such as news articles), and is less effective on prose.
- Click "Submit" button to generate an abstract.
- Experiment with different values of N to see BLEU and N-Gram calculations. These values are recalculated each time a new value for N is selected.
- ROUGE and POS Frequencies are static: they are only calculated when the submit button is clicked.
- Note: Text abstraction is a computationally expensive task. Generating a summary will take some time. Additionally, the first time the abstractor is run, it will perform a background install of several language files to your machine. These files exceed 2GB, and are required for summarization.

## Program Structure

- ./cyan_api/  
  Application web library. Contains src/main.rs (program entry point).

- ./cyan_nlg/  
  Application machine learning library. Contains functions for text tokenization and summarization.

- ./cyan_vis/  
  Application graphing & visualization library.
