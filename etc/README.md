# Running enstate with docker-compose

To get started you can clone this repository, enter this folder and run:

```sh
docker compose up
```

This will start a redis instance, an enstate instance and a prometheus instance.

You can adjust final tweaks inside of the `.env` file (see `.env.example` for an example).

## Grafana

The grafana portal is accessible at [http://localhost:3000](http://localhost:3000).

You can login with the default credentials `admin`/`admin`.
