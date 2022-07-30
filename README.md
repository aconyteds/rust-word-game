# rust-word-game

This is a word game that was built using Rust and Yew. It also include the ability to export the underlying logic as WASM.

## Getting Started

These are the steps needed to get the application running once you have cloned it to your local:

1. Install (rust)[https://www.rust-lang.org/tools/install]
2. Double check that Rust is in the environment variable, if missing add it
3. Ensure **Cargo** is installed
4. Run `rustup target add wasm32-unknown-unknown` to add the wasm target
5. Run `cargo install trunk` to install the trunk dependencies

### Running the word game
1. Run `trunk build` to build the application (this will download dependencies)
2. Run `trunk serve --open` to launch the application, and open the browser window

### Exporting the WASM
1. Run `cargo build` to install dependencies
2. Run `cargo test` to verify WASM is functioning properly
3. Run `wasm-pack build --target web` to build the binaries
5. Copy the binaries from `pkg` into a folder in your site's source code
6. Call the methods exposed in the JS to get a suggestion, validate a guess, or generate a word

## Deployment

1. run `trunk build --release` to build the application for production. (You might want to add **--public-url={path}** to the command line if you are deploying to a location other than root on your web server)
2. copy the contents of the dist folder to your web server (ensure that the path matches the one specified in the public-url option)
3. Open the browser to your server's URL
