<img src=".github/banner.png#1" alt="enstate.rs" />

## 📌 API Specification (OpenAPI)

The API specification is available at `{your_site_here}/docs`.

## 🚀 Getting Started

We believe software should be simple and containerized. Enstate provides you with a lightweight docker container that you can run anywhere.

### 🐳 Docker

```sh
docker run \
  -p 3000:3000 \
  -e REDIS_URL=redis://0.0.0.0:6379 \
  -e RPC_URL=https://rpc.ankr.com/eth \
  ghcr.io/v3xlabs/enstate:0.0.1
```

### 🐳 Docker Compose
    
```yaml
version: "3.8"
services:
  enstate:
    image: ghcr.io/v3xlabs/enstate:0.0.1
    ports:
     - 3000:3000
    environment:
     - REDIS_URL=redis://redis:6379
     - RPC_URL=https://rpc.ankr.com/eth
    depends_on:
     - redis
  redis:
    image: redis:6.2.5-alpine
    ports:
      - 6379:6379
```

## 🛣️ Roadmap

- Dockerize
- TTL Specification
- Internal Batching
- Batching endpoints
