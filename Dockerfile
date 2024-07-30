# setup lazymc versions
ARG LAZYMC_VERSION=0.2.11
ARG LAZYMC_VERSION_LEGACY=0.2.10

# build lazymc
FROM rust:1.80 as lazymc-builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/lazymc
ARG LAZYMC_VERSION
ENV LAZYMC_VERSION=$LAZYMC_VERSION
RUN git clone --branch v$LAZYMC_VERSION https://github.com/timvisee/lazymc . && \
    cargo build --release --locked

# build lazymc-legacy
FROM rust:1.80 as lazymc-legacy-builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/lazymc
ARG LAZYMC_VERSION_LEGACY
ENV LAZYMC_VERSION_LEGACY=$LAZYMC_VERSION_LEGACY
RUN git clone --branch v$LAZYMC_VERSION_LEGACY https://github.com/timvisee/lazymc . && \
    cargo build --release --locked

# build this app
FROM rust:1.80 as app-builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/lazymc-docker-proxy
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

# final image
FROM eclipse-temurin:21-jre-jammy

# setup lazymc version
ARG LAZYMC_VERSION
ENV LAZYMC_VERSION=$LAZYMC_VERSION
ARG LAZYMC_VERSION_LEGACY
ENV LAZYMC_VERSION_LEGACY=$LAZYMC_VERSION_LEGACY

# Install docker
RUN apt-get update && apt-get install -y docker.io

# Copy the compiled binary from the lazymc-builder stage
COPY --from=lazymc-builder /usr/src/lazymc/target/release/lazymc /usr/local/bin/lazymc

# Copy the compiled binary from the lazymc-legacy-builder stage
COPY --from=lazymc-legacy-builder /usr/src/lazymc/target/release/lazymc /usr/local/bin/lazymc-legacy

# Copy the compiled binary from the lazymc-docker-proxy stage
COPY --from=app-builder /usr/src/lazymc-docker-proxy/target/release/lazymc-docker-proxy /usr/local/bin/lazymc-docker-proxy

# Set the working directory
WORKDIR /app

# Run lazymc by default
ENTRYPOINT ["lazymc-docker-proxy"]