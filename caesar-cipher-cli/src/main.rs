/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/

use caeser_cipher_cli::{decrypt, encrypt};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,

    /// Whether to output the result in uppercase
    #[arg(short, long)]
    uppercase: bool,
}

// run it
fn main() {
    let args = Args::parse();
    if args.encrypt {
        let result = encrypt(&args.message, args.shift);
        if args.uppercase {
            println!("{}", result.to_uppercase());
        } else {
            println!("{}", result);
        }
    } else if args.decrypt {
        let result = decrypt(&args.message, args.shift);
        if args.uppercase {
            println!("{}", result.to_uppercase());
        } else {
            println!("{}", result);
        }
    } else {
        println!("Please specify either --encrypt or --decrypt");
    }

    println!("Thank you for using the caeser cipher CLI!");
}