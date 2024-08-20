# Get the current date and time precise to the second
DATE_TAG := $(shell date +%Y-%m-%d_%H-%M-%S)

# Docker image name
IMAGE_NAME := hpdevelop/cubejs

IMAGE_VERSION := 0.1

# Default target
all: build

# Build the Docker image with the current date tag
build:
	docker build --no-cache -t $(IMAGE_NAME):$(DATE_TAG) --progress=plain --build-arg IMAGE_VERSION=${IMAGE_VERSION} -f packages/cubejs-docker/release.Dockerfile . 2>&1 | tee build.log

# Build the Docker image with a fixed tag for development
dev:
	docker build --no-cache -t $(IMAGE_NAME):dev --progress=plain --build-arg IMAGE_VERSION=${IMAGE_VERSION} -f packages/cubejs-docker/release.Dockerfile . 2>&1 | tee build_dev.log

# Prune Docker system
prune:
	docker system prune --all --volumes -f

# Clean up all Docker images built by this Makefile
clean:
	docker rmi $$(docker images $(IMAGE_NAME) -q) 2>/dev/null || true

.PHONY: all build dev prune clean
