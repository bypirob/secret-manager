# Secret manager

Manage your local .env credentials and secrets like a pro.

## Problem

The best security practice is to avoid storing local credentials in your project. However, in reality, many developers still use .env for local development and occasionally you need to test  things so you need to have the credentials.

Nowadays, it is considered a best practice to avoid having any sensitive files inside the project to prevent any potential leaks, especially with the increasing use of AI for developing.

## Solution

I have developed this program to securely store your credentials in a separate folder, preventing any accidental exposure by AI or other tools.

Additionally, the program dynamically injects the .env variables into a child process, or you can specify a custom path.

## Usage

```
sm init
nano $(sm path)
sm run <your program>
```

## Installation

The installation process is manual:

1. Install Rust.
2. Modify the makefile INSTALL_DIR.
3. Run `make all`.
