TARGET_REPO:=chunleng/diesel-cli
cmd?=
tag?=
platform?=$(shell docker version --format '{{.Server.Os}}/{{.Server.Arch}}')

default:
	make platform=linux/arm64,linux/amd64 deploy

login:
	docker login

deploy: login
	docker buildx build --push -t ${TARGET_REPO} $(if $(tag),-t ${TARGET_REPO}:${tag},) --target runner --platform=${platform} .

run:
	docker buildx build --load -t ${TARGET_REPO} --target runner --platform=${platform} .
	docker run -it --rm ${TARGET_REPO} ${cmd}
