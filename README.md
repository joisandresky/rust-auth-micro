# Rust Auth Microservices Example
An Example of Rust Microservices for Authentication using Axum, Tonic (for gRPC), ~~JWT~~/PASETO (haven't decide yet which one).

# TODO
- [x] Axum for HTTP/REST API
- [x] Role CRUD
- [x] User Registration & Attaching Role to User
- [x] Add Some Validation
- [x] ~~JWT~~/PASETO Authentication
- [x] Redis for Token Store
- [x] Auth Middleware
- [x] Get User Info
- [x] Logout & clear redis key
- [x] gRPC API
- [x] async gRPC Interceptor
- [x] Change bcrypt to Argon2
- [x] Axum with Gracefully Shutdown
- [x] Dockerfile
- [ ] Observability
    - [ ] OpenTelemetry Integrations
    - [ ] Jaeger Tracing
    - [x] Prometheus Metrics
    - [x] Grafana Dashboard