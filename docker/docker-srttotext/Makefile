TARGET_REPO:=chunleng/srttotext
cmd?=
platform?=$(shell docker version --format '{{.Server.Os}}/{{.Server.Arch}}')

default:
	make platform=linux/arm64,linux/amd64 deploy

login:
	docker login

deploy:
	docker buildx build --push -t ${TARGET_REPO} --platform=${platform} .

run:
	docker buildx build --load -t ${TARGET_REPO} --platform=${platform} .
	docker run -it --rm ${TARGET_REPO} ${cmd}
