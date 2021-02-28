FROM ubuntu:xenial

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL io.tetsy.image.authors="devops-team@tetcoin.org" \
	io.tetsy.image.vendor="Tetsy Technologies" \
	io.tetsy.image.title="tetsy/tetsy" \
	io.tetsy.image.description="Tetsy Vapory. The Fastest and most Advanced Vapory Client." \
	io.tetsy.image.source="https://github.com/openvapory/tetsy-vapory/blob/${VCS_REF}/\
scripts/docker/hub/Dockerfile" \
	io.tetsy.image.documentation="https://wiki.tetcoin.org/Tetsy-Vapory" \
	io.tetsy.image.revision="${VCS_REF}" \
	io.tetsy.image.created="${BUILD_DATE}"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		file curl jq; \
# apt cleanup
	apt-get autoremove -y; \
	apt-get clean; \
	rm -rf /tmp/* /var/tmp/* /var/lib/apt/lists/*; \
# add user
	groupadd -g 1000 tetsy; \
	useradd -m -u 1000 -g tetsy -s /bin/sh tetsy

WORKDIR /home/tetsy

# add tetsy-vapory binary to docker image
COPY artifacts/x86_64-unknown-linux-gnu/tetsy /bin/tetsy
COPY tools/check_sync.sh /check_sync.sh

# switch to user tetsy here
USER tetsy

# check if executable works in this container
RUN tetsy --version

EXPOSE 5001 8080 8082 8083 8545 8546 8180 30303/tcp 30303/udp

ENTRYPOINT ["/bin/tetsy"]
