# This file was generated using the `makegen` cli tool
.PHONY: clean docker docker-build

clean:
	@python ./scripts/clean.py

docker: docker-build
	@docker run -d --name stormi -p 5346:6435 stormi

docker-build:
	@docker build . -t stormi
