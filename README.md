# 🏰 ConcurrentChess > A Rust Chess Engine ♟️

## 💡 Features

- 🎯 **Accurate ruleset:** Implements all standard chess rules including en passant, castling and promotions.
- 🧠 **Intelligent board management:** Uses a 2D vector for efficient and simplified board handling.
- 👑 **Versatile pieces:** Each piece type (Pawn, Rook, Knight, Bishop, Queen, King) is individually defined and handles its own move validations. 

## 🚀 Getting Started

1. Clone the repository to your local machine.
2. Run the program with `cargo run` from the root directory.

## 📖 How to Play

The game works with a command-line interface where you input your chess piece moves in algebraic notation. For example, to move your pawn from e2 to e4, type `e2 e4`. Capital letters (e.g., 'P') represent white pieces; lowercase letters (e.g., 'p') represent black pieces.

## 🔧 Configuring Rules

All the piece movement rules are maintained in a separate rules.rs file for easy modification and testing.

## 📃 License

This project is licensed under the <INSERT LICENSE> - see the [LICENSE.md](LICENSE.md) file for details.

## 📮 Get in Touch

If you have a question or found a bug, please file a GitHub issue. We welcome any feedback, ideas or contributions.

Enjoy playing real console-based Chess with Rust Chess Engine! 🎉
