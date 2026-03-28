# Deploying Newt Canvas IDE Playground

Run the Canvas IDE publicly so anyone can try Newt in a browser.

## Local Docker

```bash
docker build -t newter-playground .
docker run -p 3333:3333 newter-playground
```

Open http://localhost:3333

To serve a different example:

```bash
docker run -p 3333:3333 newter-playground \
  newter-compiler serve examples/counter.newt --port 3333
```

## Docker Compose

```bash
docker compose up
```

## Fly.io

```bash
fly launch          # first time — creates app
fly deploy          # subsequent deploys
```

The included `fly.toml` configures:
- Region: `sin` (Singapore) — change `primary_region` as needed
- Auto-stop/start machines (scale to zero when idle)
- 512 MB RAM, 1 shared CPU

## Railway

1. Connect your GitHub repo on [railway.app](https://railway.app)
2. Railway auto-detects the `Dockerfile`
3. Set port variable if needed: `PORT=3333`

## Environment variables

| Variable    | Default | Description       |
|-------------|---------|-------------------|
| `NEWT_PORT` | `3333`  | Server listen port |
