FROM julia:1.9 AS builder

WORKDIR /build

# Install dependencies
COPY smartlms-ml/Project.toml .
RUN julia -e 'using Pkg; Pkg.instantiate()'

# Copy source code
COPY smartlms-ml/src ./src

# Precompile
RUN julia -e 'using SmartLMSML'

# Production image
FROM julia:1.9-slim

WORKDIR /app

# Copy from builder
COPY --from=builder /root/.julia /root/.julia
COPY --from=builder /build/src ./src

# Copy entrypoint script
COPY smartlms-ml/docker-entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["/entrypoint.sh"]

CMD ["julia", "/app/src/api/server.jl"]