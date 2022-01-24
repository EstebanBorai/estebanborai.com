## Development

### Requirements

- NodeJS
- Rust

### Setup

#### Client

The client-side of this application is writen on Svelte using SvelteKit.
You must install dependencies using:

```bash
pnpm install
```

And then run the development server by performing:

```bash
pnpm run dev
```

### Server

The API server is written using the Rocket.rs HTTP server framework.
This server must connect to a PostgreSQL database which is
available via Docker, execute `docker-compose up --build` in
the root directory in order to start the database service.

#### Database

This project uses Diesel ORM to perform database related operations.

It's recommended to install the Diesel CLI binary using `cargo install`
to use this project.

1. Install `libpq` using Homebrew

```bash
brew install libpq && brew link --force libpq
```

2. Then add the library to your PATH

```bash
echo 'export PATH="/usr/local/opt/libpq/bin:$PATH"' >> ~/.zshrc
```

3. Finally install Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

The following sections will walk you through setting up Diesel for the project,
instead of steps these are defined as sections to easily revisit/navigate them
but you must follow them from top to bottom.

#### Initialize Schemas

Generates the `schema.rs` file.

```bash
DATABASE_URL=postgres://website-api:website-api@127.0.0.1:5432/website-api diesel database setup
```

#### Create migrations

Create a new migration file.

```bash
DATABASE_URL=postgres://website-api:website-api@127.0.0.1:5432/website-api diesel migration generate create_uuid_extension
```

#### Run migrations

Run migrations against the database instance.

```bash
DATABASE_URL=postgres://website-api:website-api@127.0.0.1:5432/website-api diesel migration run
```
