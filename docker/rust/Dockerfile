FROM rust:1.41.0 AS production-pseudo

RUN apt update && apt install -y --no-install-recommends make less && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# --

FROM production-pseudo AS development

RUN apt update && apt install -y --no-install-recommends \
      git \
      telnet \
      dnsutils \
      net-tools \
      && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

RUN rustup default nightly

RUN cargo install cargo-edit cargo-tree cargo-cache

RUN rustup component add rustfmt rust-analysis rust-src rls && \
    cargo cache --autoclean
