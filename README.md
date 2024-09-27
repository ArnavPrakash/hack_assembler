# Hack Assembler

This is an assembler for the Hack assembly language, used in the Nand2Tetris course. The assembler translates Hack assembly code (`.asm` files) into Hack machine code (`.hack` files).

## Getting Started

### Prerequisites

- Rust programming language

### Installation

1. Build the project:

    ```bash
    cargo build --release
    ```

### Usage

1. Create or obtain a Hack assembly file with the `.asm` extension.

2. Run the assembler:

    ```bash
    ./target/release/hack_assembler path/to/your/file.asm
    ```

   This will generate a corresponding `.hack` file in the same directory.
