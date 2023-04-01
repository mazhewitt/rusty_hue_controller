####################################################################################################
## Builder
####################################################################################################

ARG ARCH=aarch64-musl

FROM messense/rust-musl-cross:$ARCH AS builder

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
FROM scratch
ARG MUSL_PLATFORM=aarch64-unknown-linux-musl
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
