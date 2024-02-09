use std::path::{PathBuf};
use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::keccak256;
use std::time::{SystemTime, UNIX_EPOCH};
use web3::types::Address;
use rand_jitter::JitterRng;
use std::collections::BTreeMap;
use serde::{Deserialize,Serialize};


pub fn get_nstime() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // The correct way to calculate the current time is
    // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
    // But this is faster, and the difference in terms of entropy is
    // negligible (log2(10^9) == 29.9).
    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

#[derive(Clone, Debug, Copy)]
pub struct KeyPair {
    sec_key: SecretKey,
    pub_key: PublicKey,
}

impl KeyPair {
    pub fn generate() -> KeyPair {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = JitterRng::new_with_timer(get_nstime);
        let (secret_key, pub_key) = secp.generate_keypair(&mut rng);

        KeyPair {
            sec_key: secret_key,
            pub_key: pub_key,
        }
    }

    pub fn node_secret_key(&self) -> String {
        hex::encode(self.sec_key.secret_bytes())
    }

    pub fn node_public_key_bytes(&self) -> Vec<u8> {
        let pubkey = self.pub_key.serialize_uncompressed();
        debug_assert_eq!(pubkey[0], 0x04);
        pubkey[1..].to_vec()
    }
    pub fn node_public_key(&self) -> String {
        hex::encode(self.node_public_key_bytes())
    }

    pub fn account_public_key(&self) -> String {
        self.pub_key.to_string()
    }

    pub fn address(&self) -> Address {
        let hash = keccak256(&self.node_public_key_bytes());
        Address::from_slice(&hash[12..])
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigBlock {
    pub chain_id: u32,
    pub homestead_block: u32,
    pub dao_fork_block: u32,
    pub eip150_block: u32,
    pub eip155_block: u32,
    pub eip158_block: u32,
    pub byzantium_block: u32,
    pub constantinople_block: u32,
    pub constantinoplefixblock: u32,
    pub muir_glacier_block: u32,
    pub berlin_block: u32,
    pub london_block: u32,
    pub arrow_glacier_block: u32,
    pub gray_glacier_block: u32,
    pub zero_base_fee: u32,
    pub qbft: BTreeMap<String, u32>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub balance: String,
    pub address: String,
    pub comment: Option<String>,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

impl Allocation {
    pub fn new(balance: u128, address: String, private_key: Option<String>, public_key: Option<String>, comment: Option<String>) -> Allocation {
        Allocation {
            balance: format!("{}", balance),
            address: address,
            private_key: private_key,
            public_key: public_key,
            comment: comment,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenesisBlock {
    pub config: ConfigBlock,
    pub nonce: String,
    pub gas_limit: String,
    pub difficulty: String,
    pub number: String,
    pub gas_used: String,
    pub parent_hash: String,
    pub mix_hash: String,
    pub extra_data: String,
    pub coinbase: String,
    pub alloc: BTreeMap<String, Allocation>
}

impl GenesisBlock {
    pub fn load(path: PathBuf) -> GenesisBlock {
        let pathstr = format!("{}", path.display());
        let read = std::fs::read_to_string(path).expect(&format!("failed to read genesis block file: {}", pathstr));
        let gb: GenesisBlock = serde_json::from_str::<GenesisBlock>(&read).expect(&format!("failed to parse JSON from file: {}", pathstr));
        gb
    }
}
