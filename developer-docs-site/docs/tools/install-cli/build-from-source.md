---
title: "Build CLI from Source Code"
---

# Build Diem CLI from Source Code

If you are an advanced user and would like to build the CLI binary by downloading the source code, follow the below steps, [selecting the network branch](../../guides/system-integrators-guide.md#choose-a-network) that meets your use case. Otherwise, [install the prebuilt CLI binaries](./download-cli-binaries.md) to ease ramp up and reduce variables in your environment.

Begin by preparing your environment by following the instructions in [building Diem from source](../../guides/building-from-source), note, you can skip the last section on *Building Diem* as the instructions below build in release mode.

<details>
<summary>Linux / macOS</summary>

### Linux / macOS

#### Building the Diem CLI

1. Build the CLI tool: `cargo build --package diem --release`
1. The binary will be available in at `target/release/diem`
1. (Optional) Move this executable to a place on your path. For example: `~/bin/diem`
1. View help instructions by running `~/bin/diem help`

</details>

<details>
<summary>Windows</summary>

### Windows

#### Building diem-core
    
1. Build the CLI tool: `cargo build --package diem --release`
1. The binary will be available at `target\release\diem.exe`
1. View help instructions by running `target\release\diem.exe`

</details>
