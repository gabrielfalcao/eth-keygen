use eth_keygen;

fn main() {
    let (secret_key, pub_key) = eth_keygen::generate_keypair();

    println!("secret key: {}", hex::encode(secret_key.secret_bytes()));
    println!("public key: {}", pub_key.to_string());

    let pub_address = eth_keygen::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);
}
