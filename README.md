# Openv

Openv is a tool designed to manage **.env** files, providing additional security features. It uses 1password CLI under the hood, to automatically replace environment variables with secure credentials.

It supports multiple shells and allows for customizable command allow/deny lists. This tool uses 1password for secure credential management.


## Table of Contents

- [Openv](#openv)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
    - [Homebrew (recommended)](#homebrew-recommended)
    - [Manual Installation](#manual-installation)
    - [1Password CLI](#1password-cli)
  - [Usage](#usage)
    - [Automatic Hook Setup (recommended)](#automatic-hook-setup-recommended)
    - [Manual Hook Setup](#manual-hook-setup)
  - [Commands](#commands)
  - [Configuration](#configuration)
    - [Configuration Options](#configuration-options)
  - [Supported Shells](#supported-shells)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- **Command Wrapping**: Wrap shell commands to replace environment variables with secure credentials.
- **Shell Support**: Supports multiple shells including Bash, Zsh, Fish.
- **Configurable**: Easily configure via a TOML configuration file.

## Installation

### Homebrew (recommended)

To install Openv using Homebrew, you first need to tap the repository:
`brew tap andrea11/homebrew-formulas`

Then, install Openv:
`brew install openv`

### Manual Installation

To install Openv manually, you can download the binary from the [releases page](https://github.com/andrea11/openv/releases) and place it in a folder included in your PATH (e.g., `/usr/local/bin`).

### 1Password CLI

Openv uses the 1Password CLI to fetch secrets. Make sure you have the 1Password CLI installed and configured properly. You can find the installation instructions [here](https://developer.1password.com/docs/cli/get-started).

It is also recommended to setup the 1Password desktop app enabling the '*Integrate with 1Password CLI*' option.
Otherwise, you will need to provide the 1Password CLI with a service account token.

<details>
  <summary>1Password Settings Screenshoot</summary>

  ![1Password setting](doc/1Password%20settings.jpg)

</details>

## Usage

### Automatic Hook Setup (recommended)

To use Openv, you need to set up the appropriate hooks for your shell. Here are the steps:

1. **Bash**: `openv init bash`

2. **Zsh**: `openv init zsh`

3. **Fish**: `openv init fish`

### Manual Hook Setup

If you prefer to set up the hooks manually, edit your shell configuration file (e.g., `.bashrc`, `.zshrc`, or `config.fish`), adding the following line:

1. **Bash**:
`eval $(openv hook bash)`

2. **Zsh**:
`eval $(openv hook zsh)`

3. **Fish**:
`openv hook fish | source`

## Commands

- **execute**: Execute a command wrapped with `op run`.
- **check**: Check if a command needs to be wrapped.
- **hook**: Print the shell hook for the specified shell.
- **init**: Set up the shell hook for the specified shell.

## Configuration

Openv supports some commands out-of-the-box, for which it will automatically invoke the **op** CLI. But you can customize its behavior using a configuration file.
Openv uses a TOML configuration file to manage all settings. The configuration file can be located at `~/.openv.toml`, for a global configuration, or at the root of your project (`.openv.toml`), for a local configuration.

Example `.openv.toml`:
```toml
allow_commands = [
    "^(npm|pnpm) (run )?(start|dev|build)",
    "cargo run"
]

deny_commands = [
   "^python"
]

disable_masking = false
```

### Configuration Options

- **allow_commands**: A list of Regex patterns for shell commands that are allowed.
- **denyCommand**: A list of Regex patterns for shell commands that are denied.
- **disable_masking**: A boolean to disable terminal output secrets masking (default is `false`).

## Supported Shells

Openv supports the following shells:

- **Bash**
- **Zsh**
- **Fish**

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
