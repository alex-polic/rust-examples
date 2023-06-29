use std::{ env, fs };

extern crate crypto;
use crypto::symmetriccipher::{Encryptor};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::aes::{KeySize, ctr};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let password = &args[2];

    let file_content = read_file_content(filename);
    let encrypted_text = encrypt_text(&file_content, &password);

    write_file_content(filename, &encrypted_text);
}

fn read_file_content(filename: &String) -> String {
    println!("Reading file");

    let file_content = fs::read_to_string(filename).unwrap();

    println!("Read complete, {}", file_content);
    file_content
}

fn write_file_content(file_path: &String,file_content: &Vec<u8>) {
    let write_result = fs::write(file_path, file_content);

    match write_result {
        Ok(_) => println!("File writing finished"),
        Err(_) => println!("Error happened"),
    }
}

fn encrypt_text(plaintext: &str, password: &String) -> Vec<u8> {
    let key = password.as_bytes();
    let iv = b"0123456789012345";
    let mut encryptor = ctr(KeySize::KeySize128, key, iv);


    let mut buffer = [0; 4096];
    let mut ciphertext = Vec::new();

    let mut read_buffer = crypto::buffer::RefReadBuffer::new(plaintext.as_bytes());
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        ciphertext.extend(write_buffer.take_read_buffer().take_remaining().iter().cloned());

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    ciphertext
}
