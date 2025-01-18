# Start from a stable Debian image
FROM rust:slim-buster AS builder

# Install our box dependencies in one, easily-cached layer image
RUN apt-get update -qq && apt-get install -y curl libjemalloc2 && \
    #curl -sL https://deb.nodesource.com/setup_14.x | bash - && \
    apt-get install -y git npm python3-pip uglifyjs && \
    pip3 install toml flake8

# Move to our project directory
WORKDIR /opt/openpowerlifting
# and ensure our build directory is available
RUN mkdir -p build

# Now, typically, we want to order our COPY commands such that
# things that change most often are copied in last, so
# that we can benefit from layer caching.
# However, since meet-data is so disproportionately large,
# we COPY it in FIRST and anytime we change anything else,
# layer caching will still have this cached and will bust
# later when there's probably less than a MB left to COPY in.

# So, we'll
# COPY in data from heaviest to lightest
COPY meet-data meet-data/
COPY lifter-data lifter-data/
COPY project-data project-data/

# COPY in build instructions which shouldn't change as often as the...
COPY Cargo.* ./
COPY Makefile ./

# project code
COPY .git .git/
COPY checker checker/
COPY crates crates/
COPY server server/
COPY tests tests/

# Build it
RUN make

# Move to our server
WORKDIR /opt/openpowerlifting/server

# Expose our port so we can access it from our host machine
EXPOSE 8000

# Expose Rocket to host machine
ENV ROCKET_ADDRESS=0.0.0.0

# And we're ready to docker run
CMD ["cargo", "run", "--release"]

# Put server executable and data in a smaller image
FROM debian:stable-slim
WORKDIR /opt/openpowerlifting/
COPY --from=builder /opt/openpowerlifting/server/build .
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/opt/openpowerlifting/server", "--set-cwd", "data"]
