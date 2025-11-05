image_name := "ghcr.io/lunchtimecode/me"

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------  

run:
    cargo run

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch -- just run

# Run tests    
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    
build-binary:
    cargo build --release

build version:
    docker build -t {{image_name}}:{{version}} .

push:
    just _d_push $(just get_version)
    
_d_push version:
    docker build -t {{image_name}}:{{version}} . 
    docker push {{image_name}}:{{version}}

get_version:
    cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'
