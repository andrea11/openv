# Openv

Openv is a tool designed to manage and wrap shell commands, providing additional functionality and security features. It supports multiple shells and allows for customizable command allow/deny lists.

## Table of Contents

- [Openv](#openv)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Configuration](#configuration)
    - [Configuration Options](#configuration-options)
  - [Supported Shells](#supported-shells)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- **Command Wrapping**: Wrap shell commands to add additional functionality.
- **Allow/Deny Lists**: Customize which commands are allowed or denied.
- **Shell Support**: Supports multiple shells including Bash, Zsh, Fish.
- **Configurable**: Easily configure allow/deny lists and other settings via a TOML configuration file.

## Installation

To install Openv, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/openv.git
   cd openv
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Install the binary:
   ```sh
   cargo install --path .
   ```

## Usage

To use Openv, you need to set up the appropriate hooks for your shell. Here are the steps:

1. **Bash**:
   ```sh
   openv init bash
   ```

2. **Zsh**:
   ```sh
   openv init zsh
   ```

3. **Fish**:
   ```sh
   openv init fish
   ```

## Configuration

Openv uses a TOML configuration file to manage allow/deny lists and other settings. The configuration file is located at `~/.config/openv/config.toml`.

Example `config.toml`:
```toml
allowList = [
    "npm",
    "pnpm",
    "yarn",
    "cargo",
    "go",
]

denyList = [
    "rm -rf /",
    "sudo rm -rf /",
]

disable_masking = false
```

### Configuration Options

- **allowList**: A list of commands that are allowed.
- **denyList**: A list of commands that are denied.
- **disable_masking**: A boolean to disable terminal output masking (default is `false`).

## Supported Shells

Openv supports the following shells:

- **Bash**
- **Zsh**
- **Fish**

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.