# sweather

## Disclaimer
---
This is just a pet project used to further my skills with the Rust programming language. I'm hoping for it to be a place to get a basic understanding of how to use Iced.

## About
---
`sweather` is a super simple Weather App that's written in Rust utilizing the [Iced crate](https://github.com/hecrj/iced) for the frontend. It's currently utilizing the free [`WeatherStack`](https://weatherstack.com/) API. In order to utilize this application currently, you'll need to create a free account and set up the `.env` file with your credentials. An example of this file is found [here](.env.example).

## Requirements
---
This currently runs on Rust stable 1.48. The only hard requirement would be to have Rust installed. You can find installation instructions for Rust here: https://rustup.rs

## Setup
---
The only setup required for this project would be to get an API key for [WeatherStack](https://weatherstack.com). This API is currently free.

1. Copy `.env.example` to `.env` in the same directory
2. Modify `.env` to place your actual API Access Key for the value of `API_ACCESS_KEY`, replacing the dummy value `someapiaccesskey`.

***Note:*** `.env` is ignored by the [.gitignore](.gitignore) by default. Please take care not to leak your API key in any way.

## Usage
----
To build/run this project, it's best to use cargo.

### Build
```bash
git clone https://github.com/baileyn/sweather.git
cd sweather
cargo build
```

### Run
```bash
cargo run
```
