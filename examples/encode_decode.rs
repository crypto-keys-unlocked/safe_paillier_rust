use num_bigint::BigUint;
use num_traits::Zero;


fn string_to_biguint(message: &str) -> BigUint {
    BigUint::from_bytes_be(message.as_bytes())
}

fn biguint_to_string(number: BigUint) -> String {
    String::from_utf8(number.to_bytes_be()).expect("Invalid UTF-8 sequence")
}

fn encode_message(message: &BigUint, chunk_size: usize) -> Vec<BigUint> {
    let base = BigUint::from(2u64).pow(chunk_size as u32);
    let mut chunks = Vec::new();
    let mut remainder = message.clone();

    while remainder > Zero::zero() {
        chunks.push(&remainder % &base);
        remainder /= &base;
    }

    chunks
}

fn decode_message(chunks: Vec<BigUint>, chunk_size: usize) -> BigUint {
    let base = BigUint::from(2u64).pow(chunk_size as u32);
    let mut result = Zero::zero();
    let mut multiplier = BigUint::from(1u32);

    for chunk in chunks.iter() {
        result += chunk * &multiplier;
        multiplier *= &base;
    }

    result
}

fn main() {
    let chunk_size = 8;
    let message = "Hello, World! Will this work???";
    println!("Original Message: {}", message);

    let message_biguint = string_to_biguint(message);
    println!("Original Message: {:?}", message_biguint);

    let encoded_chunks = encode_message(&message_biguint, chunk_size);
    println!("Encoded Chunks: {:?}", encoded_chunks);


    let decoded_message = decode_message(encoded_chunks, chunk_size);
    println!("Decoded Message: {}", decoded_message);
    let final_mesage = biguint_to_string(decoded_message);

    println!("Decoded Message: {}", final_mesage);
}
