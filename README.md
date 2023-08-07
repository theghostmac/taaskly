# Taaskly - A Todo Tasks API

Taaskly is a simple Todo Tasks API built using the Rocket framework in Rust and the Diesel framework for the database.

## Features

- Create, retrieve, update, and delete tasks.
- Mark tasks as done or undone.
- Connects to a MongoDB database for task storage.

## Prerequisites

- Rust: Install Rust using [Rustup](https://www.rust-lang.org/learn/get-started).
- Docker: Install Docker to run a MongoDB database locally.

## Installation
### Running the app locally with a Dockerized Mongodb container
1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/taaskly.git
   cd taaskly
   ```

2. Rename `.env.example` to `.env` and configure the environment variables for your application.

3. Build and run the MongoDB container for local development:

   ```bash
   docker-compose -f local-docker-compose.yml up -d
   ```

4. Build and run the application:

   ```bash
   cargo run
   ```
## Running the app on Docker with a Dockerized MongoDB container.

   ```bash
   docker-compose up --build
   ```

## API Endpoints

- `GET /`: Retrieve a list of tasks.
- `POST /`: Create a new task.
- `DELETE /<id>`: Delete a task by ID.
- `GET /<id>/done`: Mark a task as done by ID.

## Usage

Use tools like Postman or `curl` to interact with the API endpoints. 
See the API documentation for details on request and response formats.
