build-image:
	docker build -t chat-api-image -f dockerfiles/build.docker .

run:
	make build-image
	docker run -it --rm -v $$(pwd)/src:/project/src \
	-p 3000:3000 \
	--entrypoint watchexec \
	chat-api-image -w src cargo run

build:
	make build-image
	mkdir -p .build
	docker run -it --rm -v $$(pwd)/src:/project/src \
	-v $$(pwd)/.build:/project/.build \
	chat-api-image sh -c \
	"cargo build -Z unstable-options --out-dir=.build"

clear:
	rm -rf .build