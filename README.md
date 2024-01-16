<img src=".github/banner.png#1" alt="enstate.rs" />

## 📌 API Specification (OpenAPI)

The API specification is available [on enstate.rs](https://enstate.rs/docs) or locally at `{your_site_here}/docs`.

## 🌐 Hosted version

For demonstration purposes (and one-off usage), a hosted instance is made available at [https://enstate.rs](https://enstate.rs) and a cloudflare worker at [https://worker.enstate.rs](https://worker.enstate.rs). This instance is provided as-is and as a gift to the community. Please do not abuse it.

### 📌 Example

> [name/luc.eth](https://worker.enstate.rs/n/luc.eth) &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; [name/rescueorg.eth](https://worker.enstate.rs/n/rescueorg.eth) &nbsp;&nbsp;&nbsp; [name/antony.sh](https://worker.enstate.rs/n/antony.sh)<br />
> [image/vitalik.eth](https://worker.enstate.rs/i/vitalik.eth)&nbsp;&nbsp;&nbsp; [name/khori.eth](https://worker.enstate.rs/n/khori.eth) &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; [name/helgesson.eth](https://worker.enstate.rs/n/helgesson.eth)

## 🚀 Getting Started

We believe software should be simple and containerized. Enstate provides you with a lightweight docker container that you can run anywhere.

### 🐳 Docker

```sh
docker run \
  -p 3000:3000 \
  -e REDIS_URL=redis://0.0.0.0:6379 \
  -e RPC_URL=https://rpc.ankr.com/eth \
  ghcr.io/v3xlabs/enstate:1.0.5
```

### 🐳 Docker Compose

```yaml
version: "3.8"
services:
    enstate:
        image: ghcr.io/v3xlabs/enstate:1.0.5
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

### 🦀 Cloudflare Worker

Running the cloudflare worker is as easy as running the following command:
Additionally, there is a hosted instance available at [worker.enstate.rs](https://worker.enstate.rs).

```sh
cd worker && pnpx wrangler deploy
```

## Contributing

### Standalone Server

```sh
cargo run -p enstate
```

### Cloudflare Worker

```sh
cd worker && pnpm dev
```
