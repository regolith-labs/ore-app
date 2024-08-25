# ORE app

Web app for ORE cryptocurrency mining.

## Running locally

This app is built with [Dioxus](https://github.com/DioxusLabs/dioxus/).

To run locally, you will need [Rust](https://www.rust-lang.org/tools/install).
Once installed, you can run:
```bash
./scripts/build.sh
```

This builds the web artifacts. You can host them locally with the following command:
```bash
(cd serve && cargo run)
```

This hosts the website on `localhost:8080`.

