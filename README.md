# ORE App

Liquid digital gold. 

## Get Started

Install:
```sh
cargo install dioxus-cli
```

Build Tailwind:
```sh
npx tailwindcss \
    -i ./input.css \
    -o ./public/tailwind.css \
    --config tailwind.config.js \
    --minify
```

Build app:
```sh
dx build
```

Serve web:
```sh
dx serve
```

Serve desktop:
```sh
dx serve --platform desktop
```
