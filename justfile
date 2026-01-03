set dotenv-load

# List available commands
default:
    just -l

alias u := update

HOST := "austin@192.168.1.121"
PORT := "222"

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

# Builds an ARM compatible docker image
[group("Build")]
build-arm:
    #!/bin/bash
    docker buildx build --platform linux/arm64/v8 --tag bl0g:${TAG:-arm} --file Dockerfile.arm .

# Deploys an instance of bl0g locally using Docker Compose
[group("Deploy")]
deploy-local:
    #!/bin/bash
    : ${TAG=$(yq '.package.version' Cargo.toml)}
    docker build --tag bl0g:${TAG} --file Dockerfile.prod . && docker compose up -d

# Builds the x86 docker image and tags it with the registry location
[group('Build')]
build-kube:
    #!/bin/bash
    : ${TAG=$(yq '.package.version' Cargo.toml)}
    docker build --tag registry:5001/bl0g:${TAG} --file Dockerfile.prod .

# Checks if the version in `./version` is already the version specified in the 
# kube-deployment file. If so, requests a new version, updates the version file
# and updates the TAG variable.
[private, no-exit-message]
check-current-version:
    #!/bin/bash
    # Get TAG from the Cargo.toml if it doesn't already exist
    : ${TAG=$(yq '.package.version' Cargo.toml)}

    # Get the IMAGE specified in the kube-deployment file. (Should be what's 
    # currently deployed in the cluster.)
    IMAGE=$(
        yq -r 'select(.metadata.name=="bl0g" and 
            .kind=="Deployment").spec.template.spec.containers[].image' \
            kube-deployment.yaml \
    )

    # Get the VERSION specific in the image.
    CURRENT_VERSION="${IMAGE##*:}"

    # Compare the what's in version to what's already deployed to the cluster.
    if [[ "$CURRENT_VERSION" == "$TAG" ]]; then
        echo ""
        echo "Current tag already deployed: $TAG"
        read -p "Enter the new version: " NEW_VERSION

        # Check that the version inputted matches the semver style.
        if [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            # Replace what's in the version file with the new version.
            cargo set-version "$NEW_VERSION"
        else
            echo "Invalid version."
            exit 1
        fi
    fi

# Updates the cluster's registry with the latest image
[private]
upload-kube:
    #!/bin/bash
    : ${TAG=$(yq '.package.version' Cargo.toml)}
    set -euo pipefail

    # Build the image
    just build-kube

    # Launch the tunnel in background
    # Map port 5001 to registry-service:5000
    ssh -L 5001:10.108.202.38:5000 austin@192.168.1.121 -p 222 -N &
    TUNNEL_PID=$!          # capture the background PID

    # Close the tunnel when the process completes or fails
    trap 'echo "Stopping tunnel…"; kill "$TUNNEL_PID" 2>/dev/null || true' EXIT INT TERM

    # Wait for the tunnel to be ready
    echo -n "Waiting for local port 5001 to be ready"
    while ! nc -z localhost 5001; do
        sleep .25
        printf "."
    done
    echo "Tunnel started (PID $TUNNEL_PID) – local port 5001 → 10.108.202.38:5000"

    # Push the image to the registry
    # Requires that `/etc/hosts` has registry 127.0.0.1
    # The hostname needs to be registry becuase that's how the ingress in the 
    # kube cluster knows to route it to the service 
    # i.e. in the cluster itself `curl -H "Host: registry"` is required
    # Docker connects to localhost:5001 and sends Host: registry:5001.
    echo "Pushing image to registry"
    docker push registry:5001/bl0g:$TAG

# Updates the cluster's image and deployment file, then applies it.
[group('Deploy')]
deploy:
    #!/bin/bash
    # Upload the latest build of the image to the internal registry, then
    # update the tag in the kube config file, send it to node0, then apply it.
    # User must be in the deploygrp on node0 to be able to create files there!
    just check-current-version \
        && just upload-kube \
        && just deploy-kube

# Updates the kube-deployment file, then applies it.
[group('Deploy')]
deploy-kube:
    #!/bin/bash
    : ${TAG=$(yq '.package.version' Cargo.toml)}

    echo "Deploying $TAG"

    # Update the tag in the kube config file, send it to node0, then apply it.
    # User must be in the deploygrp on node0 to be able to create files there and
    # tagged image must already be in the private registry!
    yq eval -i 'select(.metadata.name=="bl0g" and .kind=="Deployment").spec.template.spec.containers[].image = "10.108.202.38:5000/bl0g:'$TAG'"' kube-deployment.yaml \
        && scp -P "{{PORT}}" ./kube-deployment.yaml {{HOST}}:/opt/deploys/bl0g.yaml \
        && ssh -p "{{PORT}}" {{HOST}} "kubectl apply -f /opt/deploys/bl0g.yaml"
