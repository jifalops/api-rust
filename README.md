# API Rust

[![.github/workflows/test.yml](https://github.com/jifalops/api-rust/actions/workflows/test.yml/badge.svg)](https://github.com/jifalops/api-rust/actions/workflows/test.yml)
[![.github/workflows/deploy_lambda.yml](https://github.com/jifalops/api-rust/actions/workflows/deploy_lambda.yml/badge.svg)](https://github.com/jifalops/api-rust/actions/workflows/deploy_lambda.yml)

A template repo for building a Rust API with [poem][poem] and
[poem-openapi][poem-openapi], backed by Postgres via [sqlx][sqlx], and deployable to AWS Lambda, Docker, or Railway.

[poem]: https://github.com/poem-web/poem
[poem-openapi]: https://docs.rs/poem-openapi
[sqlx]: https://github.com/launchbadge/sqlx

## Getting started

Requires Docker, which runs Postgres via `compose.yml`. `./install.sh` installs
the Rust toolchain and the cargo tools the recipes below need, and creates `.env`.

```sh
./install.sh
just dev      # start Postgres; rerun unit tests and restart the API on change
just test-e2e # the Postgres integration tests
just --list   # everything else
```

The API serves:

| Path      | What                                            |
| --------- | ----------------------------------------------- |
| `/health` | Liveness probe, returns `ok`                    |
| `/`       | Stoplight Elements API browser                  |
| `/spec`   | OpenAPI spec as JSON                            |
| `/api`    | The API itself (`/api/auth/sign_up`, `…/me`, …) |

## Environment

Local values live in `.env`, created from `.env.example` by `install.sh`. Required at runtime:

| Variable         | Notes                                                        |
| ---------------- | ------------------------------------------------------------ |
| `DATABASE_URL`   | Postgres connection string. Also what `sqlx-cli` reads.      |
| `JWT_SECRET`     | Signs and verifies tokens. Must be set.                      |
| `PORT`           | Optional, defaults to `3000`.                                |
| `TOKEN_TTL_HOURS`| Optional, defaults to `24`.                                  |

Missing required variables panic on boot rather than failing on the first request
that needs them — see `lib/src/config.rs`.

## Structure

```
lib/src/
  api/              OpenAPI tags
  auth/             example domain: sign-up, sign-in, bearer-token verification
  user/             example domain: CRUD over a users table
  config.rs         environment, validated at startup
  database/         Created/Updated/Deleted envelopes, shared by repositories
  error.rs          AppError, InfraError, and their HTTP status mapping
  error_reporting/  one place to hook up an error-tracking backend
  init.rs           startup: infra, routers, middleware, graceful shutdown
  postgres/         pool + migrations
lib/migrations/     schema, applied automatically on connect
```

Each domain follows the same shape, so `user/` is the one to copy when adding one:

```
<domain>/
  mod.rs              re-exports only
  models.rs           domain types
  errors.rs           the domain's error enum + its HTTP status mapping
  repo.rs             the repository trait, its row DTO, and its contract tests
  service.rs          business logic over the trait
  router.rs           poem-openapi handlers
  adapters/
    repo_in_memory.rs   no database; also where the contract tests run as unit tests
    repo_postgres.rs    sqlx, queries loaded from adapters/sql/*.sql
    sql/*.sql           one file per query
```

Adding a domain means: write `errors.rs` and add a transparent `AppError` variant
for it, write the repo trait plus its `testing` module, implement both adapters,
and wire the service into `init.rs`.

### Errors

Handlers return `AppResult<T>` and use `?`. Each domain error implements poem's
`ResponseError` to map its variants to status codes, and `AppError` delegates to
whichever domain it wraps. `AppError` also implements `ApiResponse` with an empty
`meta()`, which keeps every possible error status out of the generated OpenAPI
spec — operations document their success response only.

### Database and the offline query cache

Queries use sqlx's compile-time-checked `query_file_as!`, so building normally
needs a live database. The verified query metadata is committed to `lib/.sqlx`,
and `SQLX_OFFLINE=true` builds against that instead — which is how CI and the
Docker images build with no Postgres available.

**Re-run `just sqlx-cache` whenever you change a `.sql` file, a migration, or a row
struct, and commit the resulting `lib/.sqlx` changes.** Stale cache entries fail
the build with a confusing macro error.

### Tests

Repository tests are written once, against the trait, in each domain's
`repo.rs::testing` module. Both adapters run them:

- the in-memory adapter runs them as unit tests, with no database —
  `cargo nextest run --profile unit`, which `just dev` reruns on every change;
- the Postgres adapter runs them from `lib/tests/`, where `#[sqlx::test]` creates
  a migrated scratch database per test — `just test-e2e`, gated behind the
  `testing` feature.

## Deploy

### AWS Lambda

Built and deployed with [Cargo Lambda][1]. The `lambda` feature swaps the server
for `poem-lambda`. `just lambda` runs a release build in Cargo Lambda's local
emulator. Deploys run only in CI: on pushes to `main`, when the `DEPLOY_LAMBDA`
variable is `true`, `.github/workflows/deploy_lambda.yml` cross-compiles for
arm64 and deploys the function behind a public function URL. It needs the
`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `DATABASE_URL`, and `JWT_SECRET`
secrets and the `AWS_REGION` and (optional) `RUST_LOG` variables.
Memory and timeout live under `[package.metadata.lambda.deploy]` in
`lib/Cargo.toml`.

[1]: https://www.cargo-lambda.info/

### Docker / Railway

`deploy/docker/Dockerfile` is a multi-stage build producing a distroless image.
`just docker` builds and runs it locally on port 3010; `railway.toml` points
Railway at the same file with `/health` as its healthcheck.

## Auth, and what to replace

The included auth is a self-contained example: passwords hashed with argon2 and
stored on the `users` row, tokens signed as HS256 JWTs by `AuthRepoJwt`. It is a
starting point, not a finished identity system — there is no email verification,
password reset, refresh token, or revocation. To use a managed provider instead,
implement `AuthRepo` against it and swap the adapter in `init.rs`; nothing outside
`auth/adapters/` needs to change.

## License

Zero-Clause BSD. See [LICENSE](LICENSE).
