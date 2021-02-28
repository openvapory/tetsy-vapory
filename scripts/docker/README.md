## Usage

```docker build -f docker/ubuntu/Dockerfile --tag vapcore/tetsy:branch_or_tag_name .```

## Usage - CentOS

Builds a lightweight non-root Tetsy docker image:
```
git clone https://github.com/openvapory/tetsy-vapory.git
cd tetsy-vapory
./scripts/docker/centos/build.sh
```

Fully customised build:
```
TETSY_IMAGE_REPO=my-personal/tetsy \
TETSY_BUILDER_IMAGE_TAG=build-latest \
TETSY_RUNNER_IMAGE_TAG=centos-tetsy-experimental \
./scripts/docker/centos/build.sh
```

Default values:
```
# The image name
TETSY_IMAGE_REPO - tetsy/tetsy

# The tag to be used for builder image, git commit sha will be appended
TETSY_BUILDER_IMAGE_TAG - build

# The tag to be used for runner image
TETSY_RUNNER_IMAGE_TAG - latest
```

All default ports you might use will be exposed:
```
#           secret
#      ipfs store     ui   rpc  ws   listener  discovery
#      ↓    ↓         ↓    ↓    ↓    ↓         ↓
EXPOSE 5001 8082 8083 8180 8545 8546 30303/tcp 30303/udp
```
