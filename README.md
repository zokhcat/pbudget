# Personal Budgeting App

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


## Future Todos
- *Caching*: Caching data, no experience, going to research.
- *Compression of data*: Use [zstd](https://crates.io/crates/zstd) to integrate compression of data.
- *Deployment*: Deploy to Heroku if I have enough dynos.
- *Frontend Integration*: Simple frontend in react/next.js
- *Testing*: Unit Tests and Integration Tests(I have never written tests)
- *CI/CD*: (Applies if I have already deployed) Integrate Github Actions.
- *Performance Optimization*: I have thought of few ways:
   - reducing the dependency bloat in the entities and migration directory.
   - Using [flamegraph](https://crates.io/crates/flamegraph) to profile my project.
- *Documentation in Swagger*(?): I don't see the harm having the API documentation in README.md
