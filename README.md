# Bl0g
A simple blog/personal site with an MVC architecture and new-school tech flavors.

## Stack
- [Axum](https://docs.rs/axum/latest/axum/)
- [Twailwind CSS](https://tailwindcss.com/)
- [htmx](https://htmx.org)
- [Alpine.js](https://alpinejs.dev)

## Creating a new blog post
- To create a new post run:
```shell
cargo run --bin post-maker <post-title-here>
```
> [!IMPORTANT]
> You'll need an `.env` file with an api token in plain text in the project's root.

This will create a markdown file in the `data/posts/` directory. The file's frontmatter 
contains a `draft` key which defaults to true. For the post to appar on the website,
that must be changed to false.

## Developing
- [Install and config](https://tailwindcss.com/blog/standalone-cli) the Tailwindcss cli.
- Install [cargo-watch](https://crates.io/crates/cargo-watch): 
`cargo install cargo-watch`
- Install [just](https://github.com/casey/just#packages)

Included is a bash script in a `justfile` that can be run with:
```shell
just dev
```

This will start the Axum server and Tailwind binary in watch modes so that saves
will trigger rebuilds while you're developing. On exiting this process, the Tailwind
binary will minify its outputted css.

### Building the Tailwind CSS separately 
- Run:
```shell
just build-tailwind
```

- Or as developing, in another tab run:
```shell
just run-tailwind
```
to automatically compile the tailwind as you're making changes.

## Deploying to Fly.io
- [Install flyctl](https://fly.io/docs/hands-on/install-flyctl/) to be able to deploy to [Fly.io](https://fly.io).

After the inital `flyctl launch` the following deploys can be done with:
```shell
flyctl deploy
```

