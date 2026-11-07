# Binary File Inspector

A command-line binary file inspector built in Rust that reads files and displays their contents in hexadecimal, binary, and ASCII formats.

This project was created to learn how computers represent data using bytes, how bitwise operations work, and how to read files efficiently in Rust.

## Features

- Reads any file as raw bytes.
- Displays bytes in hexadecimal, binary, and ASCII.
- Processes files in groups of 8 bytes.
- Displays file offsets in hexadecimal.
- Uses a fixed-size buffer instead of loading the entire file into memory.
- Handles partial reads and end-of-file (EOF).
- Handles file errors and invalid command-line arguments.
- Includes unit tests for the conversion functions.

## Getting Started

### Requirements

- Rust
- Cargo

### Running the Program

Clone the repository and navigate to the project directory.

```bash
git clone https://github.com/shoibDev/rust-systems.git
cd rust-systems/01-binary-file-inspector
```

Run the program using:

```bash
cargo run -- <file-path>
```

For example:

```bash
cargo run -- sample.txt
```

The `--` tells Cargo to pass the file path to the program.

To create a sample file using Git Bash:

```bash
printf 'Hello' > sample.txt
```

Then run:

```bash
cargo run -- sample.txt
```

### Example Output

For a file containing `Hello`:

```text
OFFSET   | HEX                      | BINARY                                                                   | ASCII
00000000 | 48 65 6C 6C 6F           | 01001000 01100101 01101100 01101100 01101111                          | Hello
Bytes inspected: 5
```

Each column represents:

| Column | Description |
|---|---|
| OFFSET | Starting byte position in hexadecimal |
| HEX | Hexadecimal representation of each byte |
| BINARY | Eight-bit binary representation of each byte |
| ASCII | Printable ASCII characters, or `.` for nonprintable bytes |

The program displays up to 8 bytes per row until the entire file has been inspected.

## How It Works

### 1. Reading the File

The program accepts a file path through command-line arguments and opens the file using `File::open()`.

It reads data into a fixed-size buffer:

```rust
const BYTES_PER_ROW: usize = 8;

let mut buffer = [0u8; BYTES_PER_ROW];
```

The buffer holds up to 8 bytes at a time.

The program repeatedly reads and processes bytes until it reaches the end of the file.

### 2. Binary Conversion

Each byte is converted into 8 binary digits using bitwise operations.

```rust
byte & 1
```

Extracts the rightmost bit.

```rust
byte >>= 1;
```

Shifts the remaining bits one position to the right.

For example:

```text
Decimal: 8
Binary:  00001000
```

### 3. Hexadecimal Conversion

Each byte is divided into two 4-bit values called *nibbles*.

```rust
let high = byte >> 4;
let low = byte & 0x0F;
```

The values are converted into hexadecimal characters (`0–9` and `A–F`).

For example:

```text
Decimal: 171
Binary:  10101011
Hex:     AB
```

### 4. ASCII Conversion

Printable ASCII bytes are converted into their corresponding characters.

For example:

```text
72  -> H
101 -> e
108 -> l
```

Nonprintable bytes are represented using `.`.

For example, carriage returns (`\r`) and line feeds (`\n`) appear as periods in the ASCII column.

### 5. Buffering and Partial Reads

The program uses a reusable 8-byte buffer.

A call to `read()` may return fewer bytes than requested, even before reaching EOF.

To handle this, the program continues reading into the remaining buffer space:

```rust
file.read(&mut buffer[bytes_read..])
```

This prevents previously read bytes from being overwritten.

Once 8 bytes have been collected, the program processes and displays them.

If the file ends before the buffer is filled, only the remaining valid bytes are displayed.

This keeps memory usage approximately constant regardless of file size.

## Testing

The project includes unit tests for:

- Binary conversion
- Hexadecimal conversion
- ASCII conversion

Run all tests using:

```bash
cargo test
```

The tests compare actual results against expected values using `assert_eq!()`.

For example:

```rust
assert_eq!(byte_to_hex(255), "FF");
```

If the result matches, the test passes.

You can also check the code using:

```bash
cargo fmt
cargo clippy
```

## What I Learned

Through this project, I practiced the following Rust and systems-programming concepts:

**Rust fundamentals**
- Working with arrays, vectors, strings, and characters.
- Understanding integer types such as `u8` and `usize`.
- Using references and borrowing (`&` and `&mut`).
- Working with slices and iterators.
- Using `match` expressions.
- Writing functions with different return types.

**Binary and data representation**
- Understanding bits and bytes.
- Converting decimal values into binary and hexadecimal.
- Using bitwise shifts (`>>`) and masks (`&`).
- Understanding ASCII character encoding.
- Understanding the difference between raw bytes and text.

**File handling**
- Reading raw file data.
- Using fixed-size buffers.
- Handling partial reads.
- Understanding end-of-file (EOF).
- Tracking file offsets.
- Processing files without loading them entirely into memory.

**Error handling and testing**
- Handling errors using `Result` and `match`.
- Validating command-line arguments.
- Writing unit tests using `#[test]`.
- Checking expected results with `assert_eq!()`.
- Using Cargo, Rustfmt, and Clippy.

## Project Purpose

This project is part of my journey toward learning systems programming and network programming with Rust.

It focuses on understanding how data is represented, manipulated, and processed at the byte level before moving into more advanced topics such as binary protocols, TCP sockets, and network communication.
