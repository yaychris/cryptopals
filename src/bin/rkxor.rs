extern crate cryptopals;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use cryptopals::challenge5::{rkxor_encrypt, rkxor_decrypt};

const USAGE: &'static str = "
Repeating-key XOR encryption and decryption

Usage: rkxor (-e | --encrypt) <key> <input>
       rkxor (-d | --decrypt) <key> <input>
       rkxor (-h | --help)

Options:
  -e --encrypt  Encrypt with the provided key.
  -d --decrypt  Decrypt with the provided key.
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_encrypt: bool,
    flag_decrypt: bool,
    arg_key:      String,
    arg_input:    String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let key = args.arg_key.as_ref();

    if args.flag_encrypt {
        let encrypted = rkxor_encrypt(key, &args.arg_input);

        println!("{}", encrypted);
    }

    if args.flag_decrypt {
        let decrypted = rkxor_decrypt(key, &args.arg_input);

        println!("{}", decrypted);
    }
}
