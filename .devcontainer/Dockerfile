# See: https://github.com/microsoft/vscode-dev-containers/tree/v0.140.1/containers/rust
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# mingw-w64 required for cross compilation of Rust from Debian linux to Windows
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends mingw-w64
