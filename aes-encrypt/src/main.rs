use std::{ env, fs };

extern crate crypto;
use crypto::symmetriccipher::SymmetricCipherError;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::aes::{cbc_encryptor, KeySize};
use crypto::blockmodes::{NoPadding};
use crypto::buffer::RefReadBuffer;
use crypto::buffer::RefWriteBuffer;


// Fix invalid length panic
// Handle encrypt errors
// Write encrypted file contents as text to new file

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let password = &args[2];

    let file_content = read_file_content(filename);
    let encrypted_text = encrypt_text(&file_content, &password).unwrap();

    println!("Encrypted text is {:?} with pass: {password}", encrypted_text);
}

fn read_file_content(filename: &String) -> String {
    println!("Reading file");

    let file_content = fs::read_to_string(filename).expect("There was an error reading your file");

    println!("Read complete");
    file_content
}

fn encrypt_text(file_content: &String, password: &String) -> Result<Vec<u8>, SymmetricCipherError> {
    let key = password.as_bytes();
    let iv = b"6543312";
    let mut encryptor = cbc_encryptor(KeySize::KeySize128, key, iv, NoPadding);

    let mut buffer = [0; 4096];
    let mut ciphertext = Vec::new();

    let mut read_buffer = RefReadBuffer::new(file_content.as_bytes());
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        ciphertext.extend(write_buffer.take_read_buffer().take_remaining().iter().cloned());

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(ciphertext)
}
