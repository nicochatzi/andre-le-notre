PROJECT=lenotre
PLATFORM=linux/arm64
PORT_MAP=8000:8000

test: build_for_test run_container

run: build_for_run run_container

shell:
	docker container exec -it ${PROJECT} /bin/bash

build_for_test:
	docker build -t ${PROJECT} --platform ${PLATFORM} --target test .

build_for_run:
	docker build -t ${PROJECT} --platform ${PLATFORM} --target run .

run_container:
	docker run -it --rm --platform ${PLATFORM} -p ${PORT_MAP} ${PROJECT}
