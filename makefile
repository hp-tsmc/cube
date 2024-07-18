# Get the current date and time precise to the second
DATE_TAG := $(shell date +%Y-%m-%d_%H-%M-%S)

# Docker image name
IMAGE_NAME := hpdevelop/cubejs

IMAGE_VERSION := 0.1

# Default target
all: build

# Build the Docker image with the current date tag
build:
	docker build -t $(IMAGE_NAME):$(DATE_TAG) --build-arg IMAGE_VERSION=${IMAGE_VERSION} -f packages/cubejs-docker/release.Dockerfile .

# Clean up the Docker image
clean:
	docker rmi $(IMAGE_NAME):$(DATE_TAG)

.PHONY: all build clean