#!/usr/bin/env bash
set -euo pipefail
: "${REVISION:?set authorized previous revision}"
gcloud run services update-traffic contract-current --to-revisions "$REVISION=100" --region "${REGION:?}"
