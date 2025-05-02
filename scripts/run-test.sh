#!/usr/bin/env bash
set -euo pipefail

#––– ensure .env_test exists
if [[ ! -f .env_test ]]; then
  echo "ERROR: .env_test not found"
  exit 1
fi

#––– load env vars from .env_test
set -a             # export all variables loaded
source .env_test
set +a

#––– always drop the test database on exit (cleanup)
trap 'sqlx database drop -y' EXIT

#––– create the database
echo "[1/4] creating test database…"
sqlx database create

#––– run your migrations
echo "[2/4] running migrations…"
sqlx migrate run

#––– run the full test suite
echo "[3/4] running cargo tests…"
RUST_LOG=trace TEST_LOG=1 RUST_BACKTRACE=1 cargo test | bunyan

#––– final drop happens automatically via the trap
echo "[4/4] tests complete; dropping test database…"
