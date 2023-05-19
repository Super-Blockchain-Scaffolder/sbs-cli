# bss-cli-tool
Source code for the awesome open-source scaffolding tool, _The Super Blockchain Scaffolder!_

<br/>

## Installation

The bss-cli can be installed with either cargo or npm.

```sh
cargo install bss-cli
```

_or_

```sh
npm i -g bss-cli
```

<br/>

## Usage

Run in interactive mode (prompts user for necessary information)
```
bss
```

Various flags can be passed to skip over the interactive prompts.

See all optional flags:
```sh
bss --help
```

Example usage with flag arguments:
```sh
bss -n "my-new_directory" -s "Chainlink Foundry Starter"
```
<br/>
---

## Dev Workflow
_Note: Open-source contributors wanted!_

<br/>

### 1) First, clone this repo
```bash
git clone 
```

<br/>

### 2) Run Tests
```bash
cargo test
```

<br/>

### 3) Run Locally
```
cargo run
```

Some examples of running locally with flag arguments:
```bash
cargo run -- -a
cargo run -- -c
cargo run -- -n foo
cargo run -- -n foo -s "Foundry Starter Kit"
```

<br/>

## Deploying

Deploys are currently done through the BSS Team account using the `rust-to-npm` package.

Note that we want "bss" to be the command users run but we want users to install it with "bss-cli" so we deploy with the -n flag like so:
```sh
rust-to-npm deploy -b -n bss-cli
```