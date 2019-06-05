# Start from a rust:nightly base image
FROM rustlang/rust:nightly

# Install our box dependencies in one, easily-cached layer image
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash - && \
    apt-get update -qq && \
    apt-get install -y nodejs python3-pip && \
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
COPY checker checker/
COPY modules modules/
COPY server server/

# Build it
RUN make

# Move to our server
WORKDIR /opt/openpowerlifting/server

# Overwrite the symlink'd map with the real built item
RUN cp client/build/static-asset-map.tera templates

# Expose our port so we can access it from our host machine
EXPOSE 8000

# Expose Rocket to host machine
ENV ROCKET_ADDRESS=0.0.0.0

# And we're ready to docker run
CMD ["cargo", "run", "--release"]
