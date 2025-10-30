use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rand::rngs::OsRng;

fn main() {
    // 1. GERAÇÃO DAS CHAVES
    let mut rng = OsRng;
    let bits = 2048;
    println!("Gerando o par de chaves RSA de {} bits...", bits);
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .expect("Falha ao gerar a chave privada.");
    let public_key = RsaPublicKey::from(&private_key);
    println!("Par de chaves gerado com sucesso!\n");

    // 2. CRIPTOGRAFIA
    let data_to_encrypt = b"Esta eh uma mensagem secreta!";
    println!("Mensagem Original: {}", String::from_utf8_lossy(data_to_encrypt));
    let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data_to_encrypt[..])
        .expect("Falha ao criptografar os dados.");
    println!("Mensagem Criptografada (Hex): {}\n", hex::encode(&encrypted_data));

    // 3. DESCRIPTOGRAFIA
    let decrypted_data = private_key.decrypt(Pkcs1v15Encrypt, &encrypted_data)
        .expect("Falha ao descriptografar os dados.");
    println!("Mensagem Descriptografada: {}", String::from_utf8_lossy(&decrypted_data));

    assert_eq!(&data_to_encrypt[..], &decrypted_data[..]);
    println!("\nSucesso! A mensagem foi descriptografada corretamente.");
}
