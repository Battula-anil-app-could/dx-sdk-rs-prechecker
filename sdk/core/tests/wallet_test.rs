use bip39::Mnemonic;

use dharitri_sdk::{crypto::public_key::PublicKey, data::address::Address, wallet::Wallet};

#[test]
fn test_private_key_from_mnemonic() {
    let mnemonic: Mnemonic = Mnemonic::parse_normalized("era office option undo phone pioneer siege yard pear jelly asthma various above script disorder solid swarm odor style caught absurd canal vacant prize").unwrap();

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic.clone(), 0, 0);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    assert_eq!(
        "15caac44800bab243fb52c4bbd0eaf787b0a0a14a762ce42c0ad680336422ecd",
        private_key.to_string()
    );
    assert_eq!(
        "e7d1c74453c9b0ee476836e6a4e84ed3b9af0f0ccba4455a77354b64d0dded86",
        public_key.to_string()
    );
    assert_eq!(
        "moa1ulguw3znexcwu3mgxmn2f6zw6wu67rcvewjy2knhx49kf5xaakrqqvjkw3",
        address.to_string()
    );

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0, 1);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    assert_eq!(
        "a12a98640e0a411a4580c7dee7952bf30de01d50ffffaa34d9496424cbb90f1a",
        private_key.to_string()
    );
    assert_eq!(
        "6b5fb3590d2f4afc199843fa7ba700f17c9cba7c8cca1da67087465905ef7560",
        public_key.to_string()
    );
    assert_eq!(
        "moa1dd0mxkgd9a90cxvcg0a8hfcq797fewnu3n9pmfnssar9jp00w4sq7d7ln5",
        address.to_string()
    );
}

#[test]
fn test_load_from_pem() {
    let wallet = Wallet::from_pem_file("tests/alice.pem").unwrap();
    eprintln!("wallte is {:?}", wallet);
    let addr = wallet.address();
    assert_eq!(
        addr.to_bech32_string().unwrap(),
        "moa17v5720m50jdvc046kvk5gydhlspvlpc5hqs63km9nxzgksw2kzxq3gdz89"
    );
}
