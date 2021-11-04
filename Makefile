# This file was generated using the `makegen` cli tool
.PHONY: run release clean

clean:
	@python ./scripts/clean.py

docker: docker-build
	@docker run -d --name stormi -p 5346:6435 stormi

docker-build:
	@docker build . -t stormi
