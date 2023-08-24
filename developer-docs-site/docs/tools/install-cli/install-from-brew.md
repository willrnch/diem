---
title: "Install the CLI with Brew"
---

# Install the Diem CLI with Brew

Recommended on macOS, `brew` is a package manager that allows for installing and updating packages in a single 
command.

:::tip Not supported on Windows
Brew is not supported fully on Windows
:::

## Installation

1. Ensure you have `brew` installed https://brew.sh/
2. Open a terminal and enter the following commands
```bash
    brew update        # Gets the latest updates for packages
    brew install diem # Installs the Diem CLI
```
3. You can now get help instructions by running `diem help`. You may have to open a new terminal window.
```bash
   diem help
```

## Upgrading the CLI

Upgrading the CLI with brew is very simple, simply run

```bash
  brew update        # Gets the latest updates for packages
  brew upgrade diem # Upgrades the Diem CLI
```

## Additional details

[Diem CLI homebrew Readme](https://github.com/aptos-labs/diem-core/blob/main/crates/diem/homebrew/README.md)
