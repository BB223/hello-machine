#!/usr/bin/env bash

REGISTRY="${REGISTRY:-docker.io}"
DOCKER_REPO="${DOCKER_REPO:-bb223}"
VER="${VER:?VER is not set}"
IMAGE_NAME=$(basename $(pwd))

docker build -t ${REGISTRY}/${DOCKER_REPO}/${IMAGE_NAME}:latest .

docker tag ${REGISTRY}/${DOCKER_REPO}/${IMAGE_NAME}:latest ${REGISTRY}/${DOCKER_REPO}/${IMAGE_NAME}:${VER}

docker push ${REGISTRY}/${DOCKER_REPO}/${IMAGE_NAME}:${VER}
docker push ${REGISTRY}/${DOCKER_REPO}/${IMAGE_NAME}:latest
