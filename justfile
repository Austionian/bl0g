set dotenv-load

# List available commands
default:
    just -l

alias u := update

# Script to run the Tailwind binary in watch mode
run-tailwind:
    #!/bin/bash
    echo "Starting the Tailwind binary."
    ./tailwindcss -i ./src/styles/styles.css -o ./assets/styles.css --watch

# Script to build and minify the Tailwind binary
build-tailwind:
    #!/bin/bash
    echo -e "\nMinifying css"
    sh -c './tailwindcss -i ./src/styles/styles.css -o ./assets/styles.css --minify'

build-cv:
    #!/bin/bash
    echo -e "\nBuilding resume"
    typst compile ./content/cv.typ ./assets/austin_rooks_cv.pdf
    echo -e "\nDone :)"

# Script to run the axum server in watch mode.
run-axum:
    #!/bin/bash
    echo "Starting the Axum server."

    export API_TOKEN=$API_TOKEN

    # Start cargo watch in the background
    sh -c 'cargo watch -w src -w templates -w content -x run'

# Script to run the axum server and tailwind binary in watch mode so updates
# will automatically be reflected. On exit, will minify tailwind's css.
#
# Install Just and run with `just dev`
dev:
    #!/bin/bash
    minify() {
        just build-tailwind
    }

    # Add a trap to run the minify function before exiting
    trap "minify; kill 0" SIGINT

    open 'http://127.0.0.1:1111'

    just run-axum & just run-tailwind
    TAILWIND_PID=$!

    wait $TAILWIND_PID

# Update dependencies and run the tests.
update:
    #!/bin/bash
    cargo update
    echo $'Dependencies updated!\n'
    cargo test

# Builds the docker image
docker-build:
    docker buildx build --platform linux/arm64/v8 --tag bl0g --file Dockerfile .

docker-deploy:
    DOCKER_HOST="ssh://austin@cluster.local" docker compose up -d

# Builds the new images, saves it to the pi, remotely starts it up with docker compose
deploy:
     just docker-build && docker save bl0g | bzip2 | ssh austin@cluster.local docker load && just docker-deploy
