# envs - Save and Load Environment Variables in Rust

`envs` is a command-line tool written in Rust that allows you to save and load environment variables. You can use it to:

- Save the current environment variables to a file in JSON format.
- Load and update environment variables from a previously saved JSON file.

## Features

- Save the current environment variables to a JSON file.
- Load environment variables from a JSON file and update the current environment accordingly.
- Generate a script to set or unset environment variables based on the changes.
- Works on Unix-like systems with a compatible shell (e.g., Bash, Zsh).

## Usage

### Saving Environment Variables

To save the current environment variables to a JSON file, use the `save` subcommand:

```shell
envs save <name>
```

Replace `<name>` with the desired name for the JSON file.

### Loading Environment Variables

To load environment variables from a previously saved JSON file and update the current environment, use the `load` subcommand along with `eval`:

```shell
eval "$(envs load <name>)"
```

Replace `<name>` with the name of the JSON file.

### Example

Here's an example of how to use `envs` to save and load environment variables:

```shell
# Save environment variables to a file named "myenv.json"
envs save myenv.json

# Load and update environment variables from "myenv.json"
eval "$(envs load myenv.json)"
```

## Installation

To compile and install `envs`, you need Rust and Cargo installed on your system. Follow these steps:

1. Clone this repository to your local machine:

```shell
git clone https://github.com/twwhatever/envs.git
```

2. Change into the project directory:

```shell
cd envs
```

3. Build the project:

```shell
cargo build --release
```

4. Find the compiled binary in the `target/release` directory and move it to a location in your `PATH`:

```shell
mv target/release/envs /usr/local/bin/
```

Now, you can use `envs` from the command line.

## About

[Generated with ChatGPT](https://chat.openai.com/share/17b6f1ad-8e2d-41c9-b9a0-176692ba86bc)
