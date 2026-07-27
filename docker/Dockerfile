FROM busybox:1.38.0-musl@sha256:8635836765b0c4c43970660219739baa58b0883c2e429e4b8918f7dd1519455c

ARG S6_OVERLAY_VERSION=3.2.3.2
ARG S6_ARCH=x86_64
ARG S6_NOARCH_SHA256=5379750ed30a84bbd2e2dd74847ba6b5bd29cd0b2e3ea2ec58049b57eb2eda12
ARG S6_ARCH_SHA256=e6befcc96a437a3831386ecfc51808c5d3e939dc5fe3c02ae9284599e8aa2408
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-${S6_ARCH}.tar.xz /tmp
SHELL ["/bin/ash", "-eo", "pipefail", "-c"]
RUN \
  echo "${S6_NOARCH_SHA256}  /tmp/s6-overlay-noarch.tar.xz" | sha256sum -c - && \
  echo "${S6_ARCH_SHA256}  /tmp/s6-overlay-${S6_ARCH}.tar.xz" | sha256sum -c - && \
  tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz && \
  tar -C / -Jxpf /tmp/s6-overlay-${S6_ARCH}.tar.xz && \
  rm /tmp/s6-overlay*.tar.xz && \
  ln -s /run /var/run

COPY rootfs /

ENV S6_BEHAVIOUR_IF_STAGE2_FAILS=2

EXPOSE 21115 21116 21116/udp 21117 21118 21119

HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 CMD /usr/bin/healthcheck.sh

WORKDIR /data

VOLUME /data

ENTRYPOINT ["/init"]
