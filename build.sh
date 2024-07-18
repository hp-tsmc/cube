#! /bin/bash

# build from Cube official npm packages
# docker build -t cubejs:metrics --build-arg IMAGE_VERSION=0.1 -f packages/cubejs-docker/latest.Dockerfile .
# build from local
# docker build -t cubejs:metrics --build-arg IMAGE_VERSION=0.1 -f packages/cubejs-docker/dev.Dockerfile .

docker build -t hpdevelop/cubejs:${1} --build-arg IMAGE_VERSION=0.1 -f packages/cubejs-docker/release.Dockerfile .