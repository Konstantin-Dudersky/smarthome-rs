
export def main [] {
    # webapp
    do {
        cd services/webapp
        npx tailwindcss -o ./style/output.css --minify
        trunk build --release
    }

    # db-saver
    do {
        cd ./services/db-saver
        cargo build --target=aarch64-unknown-linux-gnu --release
    }

    # deconz-ws
    do {
        cd ./services/deconz-ws
        cargo build --target=aarch64-unknown-linux-gnu --release
    }

    # http-server
    do {
        cd ./services/http-server
        cargo build --target=aarch64-unknown-linux-gnu --release
    }

    # websocket-server
    do {
        cd ./services/websocket-server
        cargo build --target=aarch64-unknown-linux-gnu --release
    }
}
