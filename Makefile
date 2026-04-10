.PHONY: help build run test docker-build docker-run docker-down

help:
	@echo "SmartLMS Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build         - Build Rust application"
	@echo "  run           - Run development server"
	@echo "  test          - Run tests"
	@echo "  docker-build  - Build Docker image"
	@echo "  docker-run    - Run Docker container"
	@echo "  docker-down   - Stop Docker containers"
	@echo "  docker-logs   - View Docker logs"
	@echo "  k8s-apply     - Apply Kubernetes manifests"
	@echo "  k8s-delete    - Delete Kubernetes resources"
	@echo "  helm-install  - Install Helm chart"
	@echo "  helm-upgrade  - Upgrade Helm release"

build:
	cd smartlms-backend && cargo build --release

run:
	cd smartlms-backend && cargo run

test:
	cd smartlms-backend && cargo test

docker-build:
	docker build -t smartlms/api:latest smartlms-backend

docker-run:
	docker-compose up -d
	docker-compose logs -f api

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs -f

k8s-apply:
	kubectl apply -f infrastructure/kubernetes/

k8s-delete:
	kubectl delete -f infrastructure/kubernetes/

k8s-logs:
	kubectl logs -n smartlms -l app=smartlms-api -f

helm-install:
	helm install smartlms ./infrastructure/helm/smartlms --namespace smartlms --create-namespace

helm-upgrade:
	helm upgrade smartlms ./infrastructure/helm/smartlms -n smartlms

# Database commands
db-migrate:
	cd smartlms-backend && cargo run --bin migrate

db-reset:
	docker-compose down -v && docker-compose up -d

# Development
dev:
	docker-compose up -d postgres-master redis
	sleep 5
	cd smartlms-backend && cargo run

# Production
prod-build:
	docker build -t smartlms/api:$(TAG) smartlms-backend
	docker tag smartlms/api:$(TAG) smartlms/api:latest

prod-push:
	docker push smartlms/api:$(TAG)
	docker push smartlms/api:latest