[![Clippy](https://github.com/osama-shawir/Caesar-Cipher-CLI-in-Rust/actions/workflows/lint.yml/badge.svg)](https://github.com/osama-shawir/Caesar-Cipher-CLI-in-Rust/actions/workflows/lint.yml)
[![Tests](https://github.com/osama-shawir/Caesar-Cipher-CLI-in-Rust/actions/workflows/tests.yml/badge.svg)](https://github.com/osama-shawir/Caesar-Cipher-CLI-in-Rust/actions/workflows/tests.yml)

## Caesar Cipher CLI Overview

### Uses

The Caesar Cipher CLI is a tool for encrypting and decrypting messages using the Caesar cipher. It's a simple and classical encryption technique where each letter in the plaintext is shifted a certain number of places down or up the alphabet.

### Command-Line Flags

- **--encrypt:**
  - *Usage:* `--encrypt`
  - *Description:* Encrypt the specified message using the Caesar cipher.

- **--decrypt:**
  - *Usage:* `--decrypt`
  - *Description:* Decrypt the specified message using the Caesar cipher.

- **--message:**
  - *Usage:* `--message "your_message"`
  - *Description:* Specify the message to be encrypted or decrypted.

- **--shift:**
  - *Usage:* `--shift 3`
  - *Description:* Specify the shift to use for the Caesar cipher. Must be between 1 and 25. The default is 3.

- **--uppercase:**
  - *Usage:* `--uppercase`
  - *Description:* Output the result in uppercase.

# Caesar Cipher CLI Usage

To use the Caesar Cipher CLI, you can either fork this repository or run it in Codespaces. Follow the steps below to encrypt and decrypt messages using the Caesar cipher.

## Clone the Repository (if not forking or using Codespaces)

```bash
git clone https://github.com/osama-shawir/Caesar-Cipher-CLI-in-Rust.git
cd caesar_cipher_cli
```

## Running the Program

To run the program, you will need to have Rust and Cargo installed on your system. You can download and install Rust and Cargo from the official website: https://www.rust-lang.org/tools/install

Once you have Rust and Cargo installed, you can run the program using the `cargo run` command. The program takes several command-line arguments that specify the message to encrypt or decrypt, the shift value to use for the cipher, and whether to encrypt or decrypt the message.

To encrypt a message, run the following command:

```bash
cargo run -- --message "Off to the bunker. Every person for themselves" --encrypt --shift 10
```
Replace the message text with the message you want to encrypt, and the shift value with the desired shift value. The program will output the encrypted message.

To decrypt a message, run the following command:

```bash
cargo run -- --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10
```

Replace the message text with the message you want to decrypt, and the shift value with the shift value used to encrypt the message. The program will output the decrypted message.

Note that the program uses the Caesar cipher, which is a simple substitution cipher that shifts each letter in the message by a fixed number of positions in the alphabet. The shift value must be between 1 and 25, and the program only works with ASCII alphabetic characters. Non-alphabetic characters are left unchanged in the output.