#########################
## rocket docker image ##
#########################
# this is a multi-stage build, it results in a clean last image with only the binary for running the api server
# to build use:
# docker build -t NAME:TAG .
# to remove intermediary containers after: docker rmi $(docker images -f "dangling=true" -q)

ENV ROCKET_APP="rocket-template"

FROM rust:1.60-bullseye as build

# copy over everything
COPY ./ ./

# build
RUN cargo build --release

# final base: slim version of debian
FROM debian:bullseye-slim

# copy build binary
COPY --from=build /$ROCKET_APP/target/release/$ROCKET_APP .

# set command to run the binary
CMD ["./${ROCKET_APP}"]
