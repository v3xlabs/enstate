####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

# Create appuser
ENV USER=enstate
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /enstate

# Copy our build
COPY ./enstate ./

# Use an unprivileged user.
USER enstate:enstate

CMD ["/enstate/enstate"]
