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

### 🦀 Cloudflare Workers

#### Running locally:

-
    ```sh
    cd worker
    ```
- Copy `.dev.vars.example` to [`.dev.vars`](https://developers.cloudflare.com/workers/configuration/environment-variables/#interact-with-environment-variables-locally) and fill in the values:

    ```sh
    # .dev.vars is Cloudflare Wrangler's .env equivalent
    cp .dev.vars.example .dev.vars
    ```
- Run the worker locally:
    ```sh
    pnpm dev
    ```
#### Deploying:

- Create a [**KV namespace**](https://developers.cloudflare.com/kv/get-started/#3-create-a-kv-namespace) via `Wrangler`:
    ```sh
    pnpm wrangler kv:namespace create <YOUR_NAMESPACE>
    ```
- From the output, copy the `id` and replace the one in [`wrangler.toml`](./worker/wrangler.toml) line 8 with it. The `binding` value should remain as `enstate-1` regardless of what you named yours when you created it,

- Deploy the worker:

    ```sh
    pnpm wrangler deploy
    ```
- Upload your secrets:
    ```sh
    echo "https://rpc.ankr.com/eth/XXXXXX" | pnpm wrangler secret put RPC_URL
    echo "XXXXX" | pnpm wrangler secret put OPENSEA_API_KEY
    ```

Additionally, there is a hosted instance available at [worker.enstate.rs](https://worker.enstate.rs).
## Contributing

### Standalone Server

```sh
cd server && cargo run -p enstate
```

### Cloudflare Worker

See [running Cloudflare Workers locally](#running-locally).
