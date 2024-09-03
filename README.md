# Nexus FPS Game

This project demonstrates a simple FPS game implemented using the Nexus zk-VM (https://nexus.xyz/). It showcases how game logic can be executed and verified in a zero-knowledge environment.

## Project Structure

- `src/main.rs`: The main program that sets up the Nexus environment, compiles the guest program, and proves/verifies the execution.
- `src/guest/src/main.rs`: The guest program that implements the game logic.
- `Cargo.toml`: The main manifest file for the project.
- `src/guest/Cargo.toml`: The manifest file for the guest program.

## Prerequisites

- Rust toolchain (version 1.77.0 or later)
- Nexus SDK (version 0.2.3)

## Building and Running

1. Clone the repository:
   ```
   git clone https://github.com/your-username/nexus-fps-game.git
   cd nexus-fps-game
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the main program:
   ```
   cargo run
   ```

## Game Logic

The game simulates a simple FPS-like environment where the player can move and shoot. The current implementation includes:

- Player movement (x and y coordinates)
- Shooting mechanics
- Ammo management
- Enemy elimination tracking

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).