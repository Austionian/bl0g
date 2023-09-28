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

This will create a markdown file in the `data/posts/` directory. The file's frontmatter 
contains a `draft` key which defaults to true. For the post to appar on the website,
that must be changed to false.

## Developing
Included is a bash script `dev.sh` in the `scripts/` directory. Make it executable:
```shell
chmod +x scripts/dev.sh
```

Then run it:
```shell
scripts/dev.sh
```

This will start the Axum server and Tailwind binary in watch modes so that saves
will trigger rebuilds while you're developing. On exiting this process, the Tailwind
binary will minify its outputted css.

### Building the Tailwind CSS separately
- [Install and config](https://tailwindcss.com/blog/standalone-cli) the Tailwindcss cli.
- Run:
```shell
./tailwindcss -i tailwind.css -o ./assets/output.css --minify
```

- Or as developing, in another tab run:
```shell
./tailwindcss -i tailwind.css -o ./assets/output.css --watch
```
to automatically compile the tailwind as you're making changes.

## Deploying to Fly.io
- [Install flyctl](https://fly.io/docs/hands-on/install-flyctl/) to be able to deploy to [Fly.io](https://fly.io).

After the inital `flyctl launch` the following deploys can be done with:
```shell
flyctl deploy
```

