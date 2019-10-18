build-image:
	docker build -t chat-api-image -f dockerfiles/build.docker .

run:
	make build-image
	docker run -it --rm -v $$(pwd)/src:/project/src \
	--entrypoint cargo \
	chat-api-image run

build:
	make build-image
	mkdir -p .build
	docker run -it --rm -v $$(pwd)/src:/project/src \
	-v $$(pwd)/.build:/project/.build \
	chat-api-image sh -c \
	"cargo build -Z unstable-options --out-dir=.build"

clear:
	rm -rf .build