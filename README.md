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
