# Enstate Server (Docker / Binary)

## Development

Copy `.env.example` to `.env` and fill in the required environment variables.

To run the caching and monitoring stack locally, you can run:

```sh
docker compose up -d
```

To test out your changes, you can run:

```sh
cargo run
```

### Metrics & Monitoring

The server exposes a `/metrics` endpoint that returns Prometheus metrics.
You can access the dashboard at [http://localhost:3000](http://localhost:3000).
