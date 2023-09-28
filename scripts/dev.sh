#!/bin/bash
# Bash script to run the axum server and tailwind binary in watch mode so updates
# will automatically be reflected. On exit, will minify tailwind's css.

minify() {
    echo "Minifying css"
    sh -c './tailwindcss -i tailwind.css -o ./assets/output.css --minify'
}

# Start cargo watch in the background
sh -c 'cargo watch -x run &'

# Add a trap to run the minify function before exiting
trap "minify; kill 0" EXIT

# Start tailwindcss in watch mode
./tailwindcss -i tailwind.css -o ./assets/output.css --watch
