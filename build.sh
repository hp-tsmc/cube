#! /bin/bash

docker build -t cubejs:metrics --build-arg IMAGE_VERSION=0.1 -f packages/cubejs-docker/latest.Dockerfile .