use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::paserk::FormatAsPaserk;
use pasetors::version4::V4;

fn main() {
    // Generate key pairs for access token, refresh token, and general token
    let keys = vec![
        ("ACCESS_TOKEN", generate_keypair()),
        ("REFRESH_TOKEN", generate_keypair()),
        ("GENERAL_TOKEN", generate_keypair()),
    ];

    println!("# Add these to your .env file:\n");

    for (name, (private_key, public_key)) in keys {
        println!("{}={}", format!("{}_PRIVATE_KEY", name), private_key);
        println!("{}={}", format!("{}_PUBLIC_KEY", name), public_key);
        println!();
    }
}

fn generate_keypair() -> (String, String) {
    // Generate the asymmetric key pair
    let keypair = AsymmetricKeyPair::<V4>::generate().unwrap();

    // Format as PASERK strings
    let mut private_paserk = String::new();
    keypair.secret.fmt(&mut private_paserk).unwrap();

    let mut public_paserk = String::new();
    keypair.public.fmt(&mut public_paserk).unwrap();

    (private_paserk, public_paserk)
}
