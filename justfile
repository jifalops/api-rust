# Task runner for this repo. Run `just` to list recipes.
# Recipes assume the tooling install.sh sets up.

set dotenv-load := true

default:
    @just --list

# Start Postgres, then rerun the unit tests and restart the API on every change.
dev: postgres
    cargo watch -x 'nextest run --profile unit' -x run

# Build the release image and run it on port 3010 against the local Postgres.
docker:
    docker compose up --build app

# Run a release build in the local Lambda emulator, served on port 9000 at
# /lambda-url/api-rust/.
lambda: postgres
    cargo lambda watch --release --features lambda

# Lint and unit tests; what CI runs on every pull request.
check:
    cargo fmt --check
    cargo clippy --workspace --all-targets --features testing -- -D warnings
    cargo test --doc --no-fail-fast
    cargo nextest run --profile unit

# Integration tests. Needs Postgres; each test gets its own scratch database.
test-e2e: postgres
    cargo nextest run --profile e2e --features testing

# Refresh the committed sqlx offline query cache after changing SQL or migrations.
sqlx-cache: postgres
    cd lib && cargo sqlx prepare

# Format, sort manifests, then apply the compiler's and clippy's own suggestions.
fix:
    cargo fmt
    cargo sort --workspace
    cargo fix --allow-dirty
    cargo clippy --fix --allow-dirty

# Start the local Postgres container and apply migrations. The app migrates on
# connect too, but the sqlx query macros need the schema before it can compile.
postgres:
    docker compose up -d --wait postgres
    sqlx migrate run --source lib/migrations
