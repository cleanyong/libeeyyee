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

    #[test]
    fn it_signs() {

        #[cfg(feature = "ecdsa")]
         use k256::{
             ecdsa::{SigningKey, Signature, signature::Signer},
             SecretKey,
         };
         use rand_core::OsRng; // requires 'getrandom' feature
        
         // Signing
         let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
         let message = b"ECDSA proves knowledge of a secret number in the context of a single message";
        
         // Note: the signature type must be annotated or otherwise inferrable as
         // `Signer` has many impls of the `Signer` trait (for both regular and
         // recoverable signature types).
         let signature: Signature = signing_key.sign(message);
        
         // Verification
         use k256::{EncodedPoint, ecdsa::{VerifyingKey, signature::Verifier}};
        
         let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
         println!("good here");
         assert!(verify_key.verify(message, &signature).is_ok());
    }
}
