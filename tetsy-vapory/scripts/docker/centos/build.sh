#!/usr/bin/env sh

# The image name
TETSY_IMAGE_REPO=${TETSY_IMAGE_REPO:-tetsy/tetsy}
# The tag to be used for builder image
TETSY_BUILDER_IMAGE_TAG=${TETSY_BUILDER_IMAGE_TAG:-build}
# The tag to be used for runner image
TETSY_RUNNER_IMAGE_TAG=${TETSY_RUNNER_IMAGE_TAG:-latest}

echo Building $TETSY_IMAGE_REPO:$TETSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")
docker build --no-cache -t $TETSY_IMAGE_REPO:$TETSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H") . -f scripts/docker/centos/Dockerfile.build

echo Creating $TETSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H"), extracting binary
docker create --name extract $TETSY_IMAGE_REPO:$TETSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")
mkdir scripts/docker/centos/tetsy
docker cp extract:/build/tetsy-vapory/target/release/tetsy scripts/docker/centos/tetsy

echo Building $TETSY_IMAGE_REPO:$TETSY_RUNNER_IMAGE_TAG
docker build --no-cache -t $TETSY_IMAGE_REPO:$TETSY_RUNNER_IMAGE_TAG scripts/docker/centos/ -f scripts/docker/centos/Dockerfile

echo Cleaning up ...
rm -rf scripts/docker/centos/tetsy
docker rm -f extract
docker rmi -f $TETSY_IMAGE_REPO:$TETSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")

echo Echoing Tetsy version:
docker run $TETSY_IMAGE_REPO:$TETSY_RUNNER_IMAGE_TAG --version

echo Done.
