# Homesteader

Homesteader is a full-stack application built with SvelteKit for the frontend and Rust for the backend. It uses PostgreSQL as the database and Docker for containerization.

## Project Structure

- `frontend/`: Contains the SvelteKit frontend application.
- `backend/`: Contains the Rust backend application.
- `docker-compose.yml`: Docker Compose configuration to run the application.

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/homesteader.git
   cd homesteader
   ```

2. Create a `.env` file in the root directory with the following content:

   ```env
   POSTGRES_USER=myuser
   POSTGRES_PASSWORD=mypassword
   POSTGRES_DB=mydatabase
   ```

3. Build and run the application using Docker Compose:
   ```bash
   docker-compose up --build
   ```

### Frontend Development

To start the frontend development server:

```bash
cd frontend
npm install
npm run dev
```

### Backend Development

To run the backend locally:

```bash
cd backend
cargo run
```

### Building for Production

To create a production build of the frontend:

```bash
cd frontend
npm run build
```

To create a production build of the backend:

```bash
cd backend
cargo build --release
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
