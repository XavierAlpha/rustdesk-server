FROM busybox:1.38.0-musl@sha256:32b5cdad7cce41dfd53d0ae06baebcf8357a147ee7694dc706911c373bc30c37 AS rootfs
RUN mkdir -p /data && chown 10001:10001 /data && chmod 0700 /data

FROM scratch
COPY --from=rootfs --chown=10001:10001 /data /data
COPY hbbs /usr/bin/hbbs
COPY hbbr /usr/bin/hbbr
COPY rustdesk-utils /usr/bin/rustdesk-utils
USER 10001:10001
WORKDIR /data
ENV HOME=/data
