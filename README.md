# sbs-cli

The Super Blockchain Scaffolder command line utility!

Learn more at https://sbs-cli.org

<br/>

## Purpose
The sbs-cli is a free and open-source command line tool meant to make it easy for anyone to explore, learn, and build with all of the many awesome blockchain tooling, libraries, and protocols available today and in the future!

<br/>

## History
The sbs-cli was created by Jim ([@JimLynchCodes](https://github.com/JimLynchCodes)) and Pranav ([@ps428](https://github.com/ps428)) for the Spring 2023 Chainlink hackathon. 

<br/>

## Companion Site
Visit [sbs-cli.org](https://sbs-cli.org) to learn more about the project and the sbs orgnaization!

<br/>

## Installation

The sbs-cli can be installed with either npm or cargo.

<br/>

### Option A) Install via NPM 
Recommended for most users
```sh
npm i -g sbs-cli
```

### Option B) Install via Cargo
Recommended for Rust users (and those contributing to sbs-cli project)
```sh
cargo install sbs-cli
```

<br/>

## Usage

Run in interactive mode (prompts user for necessary information)
```
sbs
```

Various flags can be passed to skip over the interactive prompts.

See all optional flags:
```sh
sbs --help
```

Example usage with flag arguments:
```sh
sbs -n "my-new_directory" -s "Chainlink Foundry Starter"
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

Deploys are currently done through the SBS Team account using the `rust-to-npm` package.

First, manually increment the version in `Cargo.toml` file.

Make sure you are logged into npm and cargo in your cli before trying to deploy:
```sh
cargo login
npm adduser
```

Note that we want "sbs" to be the command users run but we want users to install it with "sbs-cli" so we deploy with the -n flag like so:
```sh
rust-to-npm-cli deploy -b -n sbs-cli
```
