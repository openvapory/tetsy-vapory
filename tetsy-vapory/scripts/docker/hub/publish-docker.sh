#!/bin/sh

set -e # fail on any error

VERSION=$(cat ./tools/VERSION)
TRACK=$(cat ./tools/TRACK)
echo "Tetsy Vapory version = ${VERSION}"
echo "Tetsy Vapory track = ${TRACK}"

test "$Docker_Hub_User_Tetsy" -a "$Docker_Hub_Pass_Tetsy" \
    || ( echo "no docker credentials provided"; exit 1 )
docker login -u "$Docker_Hub_User_Tetsy" -p "$Docker_Hub_Pass_Tetsy"
echo "__________Docker info__________"
docker info

# we stopped pushing nightlies to dockerhub, will push to own registry prb.
case "${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}" in
    "$SCHEDULE_TAG")
        echo "Docker TAG - 'tetsy/tetsy:${SCHEDULE_TAG}'";
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "tetsy/tetsy:${SCHEDULE_TAG}" \
            --file tools/Dockerfile .;
        docker push "tetsy/tetsy:${SCHEDULE_TAG}";;
    "stable")
        echo "Docker TAGs - 'tetsy/tetsy:${VERSION}-${CI_COMMIT_REF_NAME}', 'tetsy/tetsy:stable'";
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "tetsy/tetsy:${VERSION}-${CI_COMMIT_REF_NAME}" \
            --tag "tetsy/tetsy:latest" \
            --tag "tetsy/tetsy:stable" \
            --file tools/Dockerfile .;
        docker push "tetsy/tetsy:${VERSION}-${CI_COMMIT_REF_NAME}";
        docker push "tetsy/tetsy:stable";
        docker push "tetsy/tetsy:latest";;
    v[0-9]*.[0-9]*)
        echo "Docker TAG - 'tetsy/tetsy:${VERSION}-${TRACK}'"
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "tetsy/tetsy:${VERSION}-${TRACK}" \
            --file tools/Dockerfile .;
        docker push "tetsy/tetsy:${VERSION}-${TRACK}";;
    *)
        echo "Docker TAG - 'tetsy/tetsy:${VERSION}-${CI_COMMIT_REF_NAME}'"
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "tetsy/tetsy:${VERSION}-${CI_COMMIT_REF_NAME}" \
            --file tools/Dockerfile .;
        docker push "tetsy/tetsy:${VERSION}-${CI_COMMIT_REF_NAME}";;
esac

docker logout
