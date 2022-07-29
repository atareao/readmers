default:
    just --list
recipe-name:
    echo "Esto no mas es una receta"
build:
    cargo build
run:
    cargo run --
clippy:
    cargo clippy
fix:
    cargo clippy --fix
# test
test:
    cargo test
