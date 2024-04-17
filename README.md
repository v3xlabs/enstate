<img src=".github/banner.png#1" alt="enstate.rs" />

## 📌 API Specification (OpenAPI)

The API specification is available [on enstate.rs](https://enstate.rs/docs) or locally at `{your_site_here}/docs`.

## 🌐 Hosted version

For demonstration purposes (and one-off usage), a hosted instance is made available at [https://enstate.rs](https://enstate.rs) and a cloudflare worker at [https://worker.enstate.rs](https://worker.enstate.rs). This instance is provided as-is and as a gift to the community. Please do not abuse it.

### 📌 Example

> [name/luc.eth](https://worker.enstate.rs/n/luc.eth) &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; [name/rescueorg.eth](https://worker.enstate.rs/n/rescueorg.eth) &nbsp;&nbsp;&nbsp; [name/antony.sh](https://worker.enstate.rs/n/antony.sh)<br />
> [image/vitalik.eth](https://worker.enstate.rs/i/vitalik.eth)&nbsp;&nbsp;&nbsp; [name/khori.eth](https://worker.enstate.rs/n/khori.eth) &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; [name/helgesson.eth](https://worker.enstate.rs/n/helgesson.eth)<br />
> [bulk/address](https://enstate.rs/bulk/a?addresses[]=0x225f137127d9067788314bc7fcc1f36746a3c3B5&addresses[]=0xd577D1322cB22eB6EAC1a008F62b18807921EFBc&addresses[]=0x8F8f07b6D61806Ec38febd15B07528dCF2903Ae7&addresses[]=0x8e8Db5CcEF88cca9d624701Db544989C996E3216&addresses[]=0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5&addresses[]=0xF1F78f308F08fDCAC933124ee8B52A376ff542B4) &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; [sse/address](https://enstate.rs/sse/a?addresses[]=0x225f137127d9067788314bc7fcc1f36746a3c3B5&addresses[]=0xd577D1322cB22eB6EAC1a008F62b18807921EFBc&addresses[]=0x8F8f07b6D61806Ec38febd15B07528dCF2903Ae7&addresses[]=0x8e8Db5CcEF88cca9d624701Db544989C996E3216&addresses[]=0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5&addresses[]=0xF1F78f308F08fDCAC933124ee8B52A376ff542B4)

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

```sh
cd worker
```

#### Run the worker locally

```sh
cp .dev.vars.example .dev.vars
```

Edit your `.dev.vars` file at this time to include environment variables for `UNIVERSAL_RESOLVER`, `RPC_URL` (optional) and `OPENSEA_API_KEY` (optional).

To run the worker locally you can now run:

```
pnpm dev
```

#### Deploying to Cloudflare Workers

Create a [**KV namespace**](https://developers.cloudflare.com/kv/get-started/#3-create-a-kv-namespace) via `wrangler` or the Cloudflare dashboard.

```sh
pnpm wrangler kv:namespace create <YOUR_NAMESPACE>
```

Copy the `id` of your newly created KV namespace to your [`wrangler.toml`](./worker/wrangler.toml). The `binding` value should remain as `enstate-1` regardless of what you named yours when you created it.

Deploy the worker:

```sh
pnpm wrangler deploy
```

Upload your secrets:

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

```sh
cd worker && pnpm dev
```

For more information on running the worker locally, please see [running Cloudflare Workers locally](#run-the-worker-locally).
