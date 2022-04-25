#########################
## rust docker image ##
#########################
# this is a multi-stage build, it results in a clean last image with only the binary for running the api server
# to build use:
# docker build --build-arg project="NAME" -t NAME:TAG .
# to remove intermediary containers after:
# docker rmi $(docker images -f "dangling=true" -q)
# to run
# docker run -it --net=host --env-file=.env NAME

FROM rust:1.60-bullseye as build

ARG project
ENV RUST_PROJECT=${project}

# new empy shell project
RUN USER=root cargo new --bin ${RUST_PROJECT}
WORKDIR /${RUST_PROJECT}

# copy over the toolchain and cargo manifest
COPY ./rust-toolchain.toml ./
COPY ./Cargo.* ./

# build dependencies (for layer caching)
RUN cargo build --release
RUN rm -r src

# copy over the source tree
COPY ./src ./src

# build
RUN /bin/bash -c 'rm -r target/release/deps/${RUST_PROJECT//-/_}*'
RUN cargo build --release

# final base: slim version of debian
FROM debian:bullseye-slim

ARG project
ENV RUST_PROJECT=${project}

# install postgres client libraries
RUN USER=root apt-get update && apt-get install postgresql-client-13 -y

# copy binary from build container
COPY --from=build /${RUST_PROJECT}/target/release/${RUST_PROJECT} .

# set command to run the binary
CMD ["sh", "-c", "./${RUST_PROJECT}"]
