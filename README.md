# Pen

**Pen** is a tool for managing Python environments with different Python versions. (v0.2.0)

## Note
I am currently seeking assistance with compiling the project for macOS. If you have experience with Rust and macOS, and would like to help, please reach out!

## Features

- Create and manage virtual environments with specified Python versions.
- Easily install, activate, and deactivate environments.
- Simple command interface for streamlined usage.

## Installation

### Manual Install (Without Building)

1. Clone the repository and navigate to the directory:
    ```bash
    git clone https://github.com/azomDev/pen.git
    cd pen
    ```

2. Set up the environment:
    ```bash
    mkdir ~/.pen
    mv files/unix/core ~/.pen/core
    mv files/unix/main.sh ~/.pen/main.sh
    printf '\n\n# pen\nalias pen=". $HOME/.pen/main.sh"\n' >> ~/.bashrc
    ```

3. Clean up and apply changes:
    ```bash
    cd ..
    rm -rf pen/
    source ~/.bashrc
    ```

### Manual Building

1. Clone the repository and navigate to the directory:
    ```bash
    git clone https://github.com/azomDev/pen.git
    cd pen
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Set up the environment:
    ```bash
    mkdir ~/.pen
    mv target/release/pen ~/.pen/core
    mv files/unix/main.sh ~/.pen/main.sh
    printf '\n\n# pen\nalias pen=". $HOME/.pen/main.sh"\n' >> ~/.bashrc
    ```

4. Clean up and apply changes:
    ```bash
    cd ..
    rm -rf pen/
    source ~/.bashrc
    ```

## Usage

**Command Format:** `pen <COMMAND>`

### Commands

- **`create`** (`c`): Create a virtual environment with a specific Python version.
    ```bash
    pen create 3.9.1
    ```

- **`install`** (`i`): Install a specific Python version.
    ```bash
    pen install 3.8.5
    ```

- **`list`** (`l`): List all installed Python versions.
    ```bash
    pen list
    ```

- **`delete`**: Delete the virtual environment in the current directory or a specific Python version.
    ```bash
    pen delete
    pen delete 3.8.5
    ```

- **`activate`** (`a`): Activate the virtual environment.
    ```bash
    pen activate
    ```

- **`deactivate`** (`d`): Deactivate the current virtual environment.
    ```bash
    pen deactivate
    ```

- **`update`**: Update Pen to the latest version.
    ```bash
    pen update
    ```

- **`uninstall`**: Uninstall Pen (does not remove virtual environments).
    ```bash
    pen uninstall
    ```

- **`help`**: Show help for Pen or a specific command.
    ```bash
    pen help
    pen help create
    ```

### Options

- **`-h`, `--help`**: Print help information.
- **`-V`, `--version`**: Print the version of Pen.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub for any bugs or suggestions. Feel free to comment on any issue if you're interested in contributing.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
