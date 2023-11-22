set dotenv-load

# Script to run the Tailwind binary in watch mode
run-tailwind:
    #!/bin/bash
    echo "Starting the Tailwind binary."
    ./tailwindcss -i tailwind.css -o ./assets/output.css --watch

# Script to build and minify the Tailwind binary
build-tailwind:
    #!/bin/bash
    echo -e "\nMinifying css"
    sh -c './tailwindcss -i tailwind.css -o ./assets/output.css --minify'

# Script to run the axum server in watch mode.
run-axum:
    #!/bin/bash
    echo "Starting the Axum server."

    export API_TOKEN=$API_TOKEN

    # Start cargo watch in the background
    sh -c 'cargo watch -x run &'

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

    just run-axum

    just run-tailwind
    TAILWIND_PID=$!

    wait $TAILWIND_PID

