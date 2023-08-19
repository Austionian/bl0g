# Bl0g
A simple blog with an MVC architecture and new-school tech flavors.
Coming soon to https://blog.r00ks.io

## Stack
- [Axum](https://docs.rs/axum/latest/axum/)
- [Twailwind CSS](https://tailwindcss.com/)
- [htmx](https://htmx.org)

## Creating a new blog post
- To create a new post in the posts directory run:
```shell
cargo run --bin post-maker <post-title-here>
```

## Building the Tailwind CSS
- [Install and config](https://tailwindcss.com/blog/standalone-cli) the Tailwindcss cli.
- Run:
```shell
./tailwindcss -i ./templates/styles/input.css -o ./assets/output.css
```

- Or as developing, in another tab run:
```shell
./tailwindcss -i ./templates/styles/input.css -o ./assets/output.css --watch
```
to automatically compile the tailwind as you're making changes.

## Deploying to Fly.io
- [Install flyctl](https://fly.io/docs/hands-on/install-flyctl/) to be able to deploy to [Fly.io](https://fly.io).

After the inital `flyctl launch` the following deploys can be done with:
```shell
flyctl deploy
```

