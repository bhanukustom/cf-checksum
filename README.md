# cf-checksum

Go binary that returns the sha256 checksum of a directory over HTTP, deployed as a Cloudflare Container.

```
client -> Worker (src/index.js) -> Durable Object (starts + proxies) -> Container (Go on :4000)
```

- `GET /health` -> `ok`
- `GET /checksum` -> `{"algo":"sha256","checksum":"…","dir":"/data","files":2}`

## 1. Run locally

```bash
CHECKSUM_DIR=data go run . &
sleep 2
curl -s localhost:4000/health
curl -s localhost:4000/checksum
kill %1
```

Cross-check the checksum independently (should match):

```bash
(cd data && for f in $(find . -type f | sort | sed 's|^\./||'); do printf '%s\0' "$f"; cat "$f"; done) | shasum -a 256
```

## 2. Run the Docker image locally

```bash
docker build -t cf-checksum .
docker run --rm -d --name cfc -p 4000:4000 cf-checksum
sleep 2
curl -s localhost:4000/checksum
docker rm -f cfc
```

## 3. Authenticate with Cloudflare

Either log in with OAuth (requests the Containers scopes automatically):

```bash
unset CLOUDFLARE_API_TOKEN
npx wrangler@latest login
npx wrangler@latest whoami
```

or use an API token with Account permissions: Workers Scripts:Edit, Cloudchamber:Edit, Workers R2 Storage:Edit, Account Settings:Read, plus User Details:Read and Memberships:Read.

Containers needs the Workers Paid plan. A 403 on `/containers/me` means the token lacks the Containers permission (or the account isn't on Paid).

## 4. Deploy

```bash
export CLOUDFLARE_ACCOUNT_ID=<account id>   # required when the login sees several accounts
npx wrangler@latest deploy
```

The output ends with the URL `https://cf-checksum.<subdomain>.workers.dev`.

## 5. Verify the deployment

```bash
npx wrangler@latest containers list
URL=https://cf-checksum.<subdomain>.workers.dev
curl -s $URL/health      # first call is slow: container cold start
curl -s $URL/checksum    # same checksum as step 1
npx wrangler@latest tail # live logs
```

If `/checksum` errors right after the first deploy, wait ~1 minute and retry.

## Rust version

Same API, built from [rust/](rust/) (own `wrangler.jsonc`, deploys as the separate Worker `cf-checksum-rust`; reuses `src/index.js`).

```bash
# run locally
(cd rust && CHECKSUM_DIR=../data cargo run --release) &
sleep 5; curl -s localhost:4000/checksum; kill %1

# docker (build context is the repo root, so data/ is reachable)
docker build -f rust/Dockerfile -t cf-checksum-rust .
docker run --rm -d --name cfc -p 4000:4000 cf-checksum-rust
sleep 2; curl -s localhost:4000/checksum; docker rm -f cfc

# deploy / verify / clean up
cd rust
npx wrangler@latest deploy
npx wrangler@latest containers list
npx wrangler@latest delete
```

URL: `https://cf-checksum-rust.<subdomain>.workers.dev`.

## Machine size

Set `instance_type` in the container entry of `wrangler.jsonc` (default `lite`), then redeploy.

| Name | vCPU | Memory | Disk |
|---|---|---|---|
| lite | 1/16 | 256 MiB | 2 GB |
| basic | 1/4 | 1 GiB | 4 GB |
| standard-1 | 1/2 | 4 GiB | 8 GB |
| standard-2 | 1 | 6 GiB | 12 GB |
| standard-3 | 2 | 8 GiB | 16 GB |
| standard-4 | 4 | 12 GiB | 20 GB |

Custom: `"instance_type": { "vcpu": 1, "memory_mib": 2048, "disk_mb": 4000 }`.

## Clean up

```bash
npx wrangler@latest delete
```
