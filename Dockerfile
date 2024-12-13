# This file is not intended to be used directly ;
# please refer to the build script instead

FROM debian:bookworm-slim

# These two are provided by docker buildx
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT

WORKDIR /app
COPY target/building-for-docker/artifacts/${TARGETOS}/${TARGETARCH}/${TARGETVARIANT}/tapo-rest ./

ENTRYPOINT ["./tapo-rest", "/app/devices.json", "--port=80"]
