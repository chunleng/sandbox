# TODO change the target repository to deploy to
TARGET_REPO:=chunleng/??
cmd?=
platform?=$(shell docker version --format '{{.Server.Os}}/{{.Server.Arch}}')

default:
	make platform=linux/arm64,linux/amd64 deploy

login:
	docker login

deploy: login
	docker buildx build --push -t ${TARGET_REPO} --platform=${platform} .

run:
	docker buildx build --load -t ${TARGET_REPO} --platform=${platform} .
	docker run -it --rm ${TARGET_REPO} ${cmd}
