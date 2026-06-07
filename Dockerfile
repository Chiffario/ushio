FROM alpine:3.22 AS runtime
ARG TARGETARCH
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY dist/${TARGETARCH}/ushio /usr/local/bin/ushio

CMD ["ushio"]
