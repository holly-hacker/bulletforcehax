#/bin/bash
cargo test --manifest-path=libs/photon_core/Cargo.toml && \
cargo test --manifest-path=libs/photon_derive/Cargo.toml && \
cargo test --manifest-path=libs/photon/Cargo.toml && \
cargo test
