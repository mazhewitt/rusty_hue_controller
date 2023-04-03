####################################################################################################
## Builder
## docker buildx -t mazhewitt/rusty_hue_controller:0.1 --build-arg MUSL_PLATFORM=armv7-unknown-linux-musleabi --build-arg ARCH=armv7-musleabi --build-arg DEPLOY_PLATFORM=linux/arm/v7 --platform linux/arm/v7 .
####################################################################################################
ARG DEPLOY_PLATFORM
ARG ARCH

FROM --platform=linux/arm64 messense/rust-musl-cross:$ARCH AS builder

# Create appuser
ENV USER=hue_controller
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /build

ADD src src
ADD Cargo.lock .
ADD Cargo.toml .


RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################


FROM --platform=$DEPLOY_PLATFORM scratch

ARG MUSL_PLATFORM
# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /build/target/$MUSL_PLATFORM/release/register_hue ./
COPY --from=builder /build/target/$MUSL_PLATFORM/release/rusty_hue_server ./
ADD Rocket.toml .

# Use an unprivileged user.
USER ${USER}:${USER}

EXPOSE 8080/tcp

ENV PATH="/app:${PATH}"

ENTRYPOINT ["/app/rusty_hue_server"]
