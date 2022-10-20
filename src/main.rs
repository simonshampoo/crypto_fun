use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let alice_private = RsaPrivateKey::new(&mut rng, bits).expect("fail");
    let alice_public = RsaPublicKey::from(&alice_private);

    let msg = b"From Bob, xoxo";

    let encrypted_msg = alice_public.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), msg);

    let decrypted_msg = alice_private.decrypt(
        PaddingScheme::new_pkcs1v15_encrypt(),
        &encrypted_msg.unwrap(),
    );

    assert_eq!(&msg, &&decrypted_msg.unwrap()[..]);
}
