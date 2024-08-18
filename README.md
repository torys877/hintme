# ChatGPT Cli Assistant

This is a simple console application that suggests the required Linux command on user request using ChatGPT.

## Prerequisites

It was build and tested with:

+ Rust `1.80.0`
+ Ubuntu `22.04`

### Create OpenAI Project And Obtain Api Key for authentification

https://platform.openai.com/docs/quickstart

## Clone

```bash
git clone git@github.com:torys877/hintme.git
cd hintme

```

## Build

### Set Data In `.env` File

```bash
cp example.env .env
```

Set your API key into `OPENAI_API_KEY` variable in `.env` file

### Build Executable File

```bash
cargo build --release
```

## Installation

### Copy Executable File And Create Symlink

```
sudo cp ./target/release/hintme /usr/bin
sudo ln -s /usr/bin/hintme /usr/local/bin
```

### Set Environment Variables For Api

```bash
export OPENAI_API_KEY="YOUR_API_KEY"
export OPENAI_API_URl="https://api.openai.com/v1/chat/completions"```
``````

## Usage

### Run Command With Question

```bash
hintme how to find all files with extension "torrent", output their paths and delete them?
```

### Expected Response

```bash
find /path/to/directory -type f -name "*.torrent" -exec rm {} +
```
