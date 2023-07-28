# Author: Chase Ruskin
# Details:
#   A series of commands to run at convenience.

# Install the debug build in placement of old binary for local system testing
install:
    cargo build
    cp ./target/debug/orbit "$HOME/.cargo/bin/orbit"

# Run partial section of tests by specifying the modules in MODS
codev MODS:
    cargo watch -c -x check -x "test -- {{ MODS }}" --ignore test/data

# Generate strings in Rust from the docs to provide offline documentation in the tool
rsman:
    python ./tools/rsmangen.py

# Synchronize documentation from TOML file to markdown and Rust
mansync:
    python ./tools/mansync/mangen.py

# Generate a summary of changelog notes for the next version release
chlog:
    python ./tools/clgen.py --verbose

# Run the documentation book on the local host for testing before going live
docs:
    mdbook serve ./docs

# Run all the possible tests available for this project
fulltest:
    cargo check
    cargo test
    python -m unittest discover -s ./tools -p '*.py'

# Sort the glossary contents
sortgloss:
    python ./tools/sort_gloss.py