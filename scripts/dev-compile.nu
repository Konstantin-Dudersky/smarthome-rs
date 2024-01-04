
export def main [] {
    # db-saver
    do {
        cd ./services/db-saver
        cargo update
        cargo build --target=aarch64-unknown-linux-gnu --release
    }

    # deconz-ws
    do {
        cd ./services/deconz-ws
        cargo update
        cargo build --target=aarch64-unknown-linux-gnu --release
    }

    # http-server
    do {
        cd ./services/http-server
        cargo update
        cargo build --target=aarch64-unknown-linux-gnu --release
    }

    # websocket-server
    do {
        cd ./services/websocket-server
        cargo update
        cargo build --target=aarch64-unknown-linux-gnu --release
    }

    # webapp
    do {
        cd services/webapp
        cargo update
        npx tailwindcss -o ./style/output.css --minify
        trunk build --release
    }
}
