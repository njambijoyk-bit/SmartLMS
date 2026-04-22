# SmartLMS Deployment Guide

## Prerequisites

- Docker & Docker Compose (for local development)
- Kubernetes 1.24+ (for production)
- Helm 3 (for Kubernetes deployment)
- PostgreSQL 16+
- Redis 7+

## Local Development

```bash
# Clone and setup
cp .env.example .env
# Edit .env with your configuration

# Start with Docker Compose
docker-compose up -d

# View logs
docker-compose logs -f api
```

## Production Deployment

### Using Helm (Recommended)

```bash
# Add Helm repository
helm repo add smartlms https://charts.smartlms.example.com
helm repo update

# Install
helm install smartlms smartlms/smartlms \
  --namespace smartlms \
  --create-namespace \
  --set config.jwt-secret=your-secret

# Upgrade
helm upgrade smartlms smartlms/smartlms -n smartlms
```

### Using Kubernetes Manifests

```bash
# Apply manifests
kubectl apply -f infrastructure/kubernetes/
```

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `REDIS_URL` | Redis connection string | `redis://localhost:6379` |
| `JWT_SECRET` | Secret for JWT signing | Required |
| `RUST_LOG` | Logging level | `info` |
| `MAX_UPLOAD_SIZE` | Max upload size in bytes | 50MB |

## Environment Variables

See `.env.example` for all configuration options.

## Health Checks

- API: `GET /health`
- Metrics: `GET /metrics`

## Scaling

The API is horizontally scalable. Use the HorizontalPodAutoscaler:
```bash
kubectl autoscale deployment smartlms-api --cpu-percent=70 --min=3 --max=10
```