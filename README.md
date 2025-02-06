# Homesteader

**Work in Progress**

Homesteader is a personal project aimed at creating a comprehensive system for managing and monitoring various aspects of a homestead. The project is currently in its early stages, with plans to eventually open-source it for community contributions.

## Services

### Backend

The backend service is built using Rust and provides the core functionality of the system. It handles data processing, storage, and communication with various sensors and devices.

### Frontend

The frontend service is developed using Svelte and provides a user-friendly interface for interacting with the system. It allows users to view and manage data collected from sensors and devices.

### Weather API

The weather API service integrates with external weather data providers to fetch and display weather information relevant to the homestead.

## Future Plans

### LoRaWAN Gateway

In the future, sensors will communicate with the system through a local LoRaWAN Gateway. This will enable long-range, low-power communication with various sensors deployed around the homestead.

### Device Testing

Currently, devices are being used for testing the backend functionality only. As the project progresses, more devices and sensors will be integrated into the system.

## Getting Started

To start the entire system, you can use Docker Compose. Ensure you have Docker and Docker Compose installed on your machine.

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/homesteader.git
   cd homesteader
   ```

2. Start the services using Docker Compose:

   ```sh
   docker-compose up
   ```

3. Access the frontend at `http://localhost:3000` and the backend at `http://localhost:8000`.

## Contributing

This project is currently for personal use, but the plan is to eventually allow open-source contributions. Stay tuned for updates on how you can contribute to the project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
