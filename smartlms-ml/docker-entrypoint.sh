#!/bin/bash
set -e

echo "Starting SmartLMS ML Engine..."

# Load Julia packages
julia -e 'using Pkg; Pkg.instantiate()' 2>/dev/null || true

# Start server
exec "$@"