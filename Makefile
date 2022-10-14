BINARY = hello-gitops-rust
PACKAGE_PATH ?= target/debug
HELLO_GITOPS_RUST = ${PACKAGE_PATH}/${BINARY}
HELLO_GITOPS_RUST_CONTAINER_NAME = ${BINARY}
BINS = ${HELLO_GITOPS_RUST}
VERSION = $(shell jq -r '.version' <package.json)
BUILD_TIME ?= $(shell date -u '+%Y%m%d%H%M%S')
BIN_DIR ?= ${PACKAGE_PATH}
REGISTRY ?= jmv2
TAG ?= ${VERSION}
REPO_INFO = $(shell git config --get remote.origin.url)
IMGNAME ?= hello-gitops-rust
IMAGE = $(REGISTRY)/$(IMGNAME)
COMMIT_SHA ?= $(shell git rev-parse --short HEAD)

build: ${BINS}

${BINS}: ${BIN_DIR} echo-build-data
	cargo build

echo-build-data:
	@echo "build_time: ${BUILD_TIME}"
	@echo "   version: ${VERSION}"

${BIN_DIR}:
	mkdir -p ${BIN_DIR}

clean:
	-rm -rf ${HELLO_GITOPS_RUST}
	docker image rm -f ${IMAGE}:${TAG} || true

container:
	@docker image inspect ${IMAGE}:${TAG} >/dev/null 2>&1 || \
		docker build -t ${IMAGE}:${TAG} .

lint:
	cargo check

run: build
	${BIN_DIR}/${BINARY}


run-container: container
	docker rm -f $(BINARY) >/dev/null 2>&1 || true
	docker run \
		-p 8080:8080 \
		--name $(HELLO_GITOPS_RUST_CONTAINER_NAME) \
		${IMAGE}:${TAG}

push-image:
	@echo Pushing image -> ${REGISTRY} ${IMAGE}
	docker image tag ${IMAGE}:${VERSION} ${IMAGE}:latest
	docker image push ${IMAGE}:${VERSION}
	docker image push ${IMAGE}:latest
