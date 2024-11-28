

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


r:
    cargo install --path . --force


export ROCKET_port := "12500"
rr:
    me

image_name := "ghcr.io/lunchtimecode/me"
    
build version:
    docker build -t {{image_name}}:{{version}} .