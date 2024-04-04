FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# System updates
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y dist-upgrade \
    && apt-get -y install --no-install-recommends \
    git make

USER vscode

# Install ASDF
RUN git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.11.3; \
    echo '. $HOME/.asdf/asdf.sh' >> ~/.bashrc; \
    echo '. $HOME/.asdf/completions/asdf.bash' >> ~/.bashrc;

ENV PATH="$PATH:/home/vscode/.asdf/bin/:/workspaces/website/node_modules/.bin"

# Install ASDF plugins
RUN asdf plugin add python; \
    asdf plugin add nodejs https://github.com/asdf-vm/asdf-nodejs.git;

WORKDIR /workspaces/fhir_prescription_creator
ADD .tool-versions /workspaces/website/.tool-versions
ADD .tool-versions /home/vscode/.tool-versions

RUN asdf install; \
    asdf reshim python; \
    asdf reshim nodejs;

# Install pip
RUN curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py \
    && python3 get-pip.py \
    && rm get-pip.py

# Install rust toolchains
RUN rustup toolchain install nightly \
    && rustup override set nightly

# Install Leptos
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install leptosfmt
