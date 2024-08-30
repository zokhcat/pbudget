# Personal Budgeting API

[![Architecture](http://img.youtube.com/vi/xa3ZMVu8dJ8/0.jpg)](http://www.youtube.com/watch?v=xa3ZMVu8dJ8?t=35s "Personal Budget API Architecture")

**[Document](https://zokhcat.notion.site/Personal-Budget-API-pbudget-cc8a5ce63fe34685b5be0551231c87a8)**

## Tech Stack

- **Actix Web**: web server framework for the microservice
- **Sea ORM**: ORM for Database(SQLite)
- **JWT Authentication**: For token-based Authentication

## Endpoints

- **POST /api/register**: Register a new user.
- **POST /api/login**: Log in and receive a JWT token.
- **GET /api/profile**: Get the profile information of the logged-in user.
- **PUT /api/profile**: Update the profile information of the logged-in user.
- **GET /api/budgets**: Get all budgets for the logged-in user.
- **POST /api/budgets**: Create a new budget.
- **GET /api/budgets/{id}**: Get a specific budget by ID.
- **PUT /api/budgets/{id}**: Update a specific budget by ID.
- **DELETE /api/budgets/{id}**: Delete a specific budget by ID.
- **GET /api/budgets/{id}/expenses**: Get all expenses for a specific budget.
- **POST /api/budgets/{id}/expenses**: Create a new expense for a specific budget.
- **GET /api/budgets/{id}/expenses/{expense_id}**: Get a specific expense by ID.
- **PUT /api/budgets/{id}/expenses/{expense_id}**: Update a specific expense by ID.
- **DELETE /api/budgets/{id}/expenses/{expense_id}**: Delete a specific expense by ID.

## Future Todos(v0.1.1)

- [x] _Caching_: Implemented caching using [redis](https://redis.io/), fairly a side quest.
- [x] _Compression of data_: Went with [default compressor](https://actix.rs/docs/response/#content-encoding) instead of zstd, which was giving messy semantic bugs.
- _Deployment_: Deploy to Heroku if I have enough dynos.
- _Frontend Integration_: Simple frontend in react/next.js
- _Testing_: Unit Tests and Integration Tests(I have never written tests)
- [x] _Containerize_: Containerize using Docker
- _Performance Optimization_: I have thought of few ways:
  - reducing the dependency bloat in the entities and migration directory.
  - Using [flamegraph](https://crates.io/crates/flamegraph) to profile my project.
- _Documentation in Swagger_(?): I don't see the harm having the API documentation in README.md
