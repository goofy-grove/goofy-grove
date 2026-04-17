# Goofy Grove

blah-blah-blah chat

## Server (development)

For working with `sea-orm-cli` you should create the `.env` file with `DATABASE_URL` variable. For example:

```sh
DATABASE_URL="sqlite://goofy_grove.sqlite?mode=rwc"
```

## Frontend (development)

To connect frontend to the server you must create `.env` file in `frontend` directory and paste something like this:

```sh
VITE_API_URL="http://{HOST}:{PORT}/api/v1"
VITE_SOCKET_URL="ws://{HOST}:{PORT}"
```

Where the
- `{HOST}` is a host from `config.yml` or default (`127.0.0.1`),
- `{PORT}` is a port from `config.yml` or default (`3003`)