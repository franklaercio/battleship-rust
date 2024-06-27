# Battleship Game Documentation

## Table of Contents

1. [Project Overview](#project-overview)
2. [Tasks and Responsibilities](#tasks-and-responsibilities)
3. [Setup and Installation](#setup-and-installation)
4. [Usage](#usage)
5. [License](#license)

## Project Overview
The Battleship Game is a classic strategy game implemented in Rust. It is designed to be played by two players on different machines, providing an engaging and interactive experience. The project is divided into several tasks, each handled by different team members to ensure a modular and efficient development process.

## Tasks and Responsibilities

### Server - Frank
Description: Implement the server-side logic to handle game communication between two players.

Responsibilities:

- Set up a server that can manage connections from two clients.
- Handle the communication protocol to send and receive game data.
- Ensure synchronization between the two players' game states.

### Show board game - Frank
Description: Develop the functionality to display the game board to each player.

Responsibilities:

- Create a representation of the game board.
- Update the board display based on player actions.
- Ensure the board is correctly rendered on each player's screen.

### Place ships - João
Description: Implement the logic to manage the locations of ships on the board.

Responsibilities:

- Allow players to place ships on the board.
- Track the positions of ships.
- Update the board when ships are placed or hit.

### Select Target - João
Description: Develop the functionality for players to select targets on the opponent's board.

Responsibilities:

- Provide a mechanism for players to choose a target location.
- Validate the chosen target.
- Send the target information to the server for processing.

### Sistema de Vidas - Moisés
Description: Implement the life system to track each player's remaining ships.

Responsibilities:

- Track the number of ships each player has left.
- Update the life count when a ship is hit.
- Ensure the life system integrates seamlessly with the rest of the game logic.

### Condição de Vitória - Moisés
Description: Develop the logic to determine when a player has won the game.

Responsibilities:

- Check the game state to determine if a player has won.
- Handle the end-game scenario, including notifying players and ending the game session.
- Ensure the victory conditions are clearly defined and correctly implemented.

## Setup and Installation

1. Clone the repository:

```sh
git clone https://github.com/franklaercio/battleship-rust.git
```

2. Navigate to the project directory:

```sh
cd battleship-rust
```

3. Build the project:

```sh
cargo build
```

4. Run the project:

```sh
cargo run
```

## Usage

- Start the server by running the main application.
- Connect two clients to the server.
- Follow the on-screen prompts to play the game.
- Each player can place bombs, select targets, and see the game board update in real-time.

## License
This project is licensed under the MIT License. See the LICENSE file for more information.
