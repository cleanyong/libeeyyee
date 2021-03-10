#[macro_use]
extern crate rand_core;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use x25519_dalek::{EphemeralSecret, PublicKey};
        use x25519_dalek::{StaticSecret};
        use rand_core::OsRng;


        let alice_secret = StaticSecret::new(OsRng);
        let alice_public = PublicKey::from(&alice_secret);

        let bob_secret = StaticSecret::new(OsRng);
        let bob_public = PublicKey::from(&bob_secret);

        let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
        println!("{:x?}", alice_shared_secret.to_bytes());

        let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
        println!("{:x?}", bob_shared_secret.to_bytes());

        assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
    }

    #[test]
    fn it_exists() {
        println!("good here");

    }

}
