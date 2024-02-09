use clap::Parser;
use eth_keygen;
use std::path::{PathBuf};
use iocore::exceptions::Exception;
use eth_keygen::KeyPair;
use std::fs::{create_dir_all, File};
use std::io::Write;


#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Params {
    #[arg(short = 'd', long, help="directory where to output the generated keys")]
    output_dir: Option<PathBuf>,

    #[arg(short, long, default_value="config/besu/genesis.json")]
    genesis_block_json_path: PathBuf,

    #[arg(short = 'S', long, default_value="nodeKey")]
    node_secret_key_filename: String,

    #[arg(short='P', long, default_value="nodeKey.pub")]
    node_public_key_filename: String,

    #[arg(short = 'p', long, default_value="accountPrivateKey")]
    account_public_key_filename: String,

    #[arg(short, long, default_value="address", help="address of wallet or node")]
    address_filename: String,

    #[arg(short, long, default_value="1000000000000000000000000000", help="initial balance of this address in the genesis block")]
    balance: u128,

    #[arg(short, long)]
    quiet: bool,

    #[arg(short, long)]
    write: bool,

    #[arg(short, long)]
    overwrite: bool,
}

impl Params {
    pub fn account_public_key(&self) -> String {
        self.account_public_key_filename.clone()
    }
    pub fn node_public_key(&self) -> String {
        self.node_public_key_filename.clone()
    }
    pub fn node_secret_key(&self) -> String {
        self.node_secret_key_filename.clone()
    }
    pub fn address(&self) -> String {
        self.address_filename.clone()
    }
    pub fn ensure_output_path_exists(&self) -> Result<PathBuf, Exception> {
        let acp = iocore::absolutely_current_path()?;
        let path = match &self.output_dir {
            Some(path) => path,
            None => &acp
        }.as_path();
        create_dir_all(path)?;
        Ok(path.to_path_buf())
    }

    pub fn write_node_secret_key(&self, key_pair: KeyPair) -> Result<PathBuf, Exception> {
        if !self.quiet {
            println!("{}: {}", &params.node_secret_key_filename, key_pair.node_secret_key());
        }
        let path = self.ensure_output_path_exists()?;
        let path = path.as_path().join(self.node_secret_key());
        if path.try_exists()? && !self.overwrite {
            Err(Exception::IOError(format!("{} already exists", path.display())))
        } else {
            let mut node_secret_key_file = File::create(path.to_path_buf())?;
            node_secret_key_file.write_all(key_pair.node_secret_key().as_bytes())?;
            Ok(path.to_path_buf())
        }
    }

    pub fn write_node_public_key(&self, key_pair: KeyPair) -> Result<PathBuf, Exception> {
        if !self.quiet {
            println!("{}: {}", &params.node_public_key_filename, key_pair.node_public_key());
        }
        let path = self.ensure_output_path_exists()?;
        let path = path.as_path().join(self.node_public_key());
        if path.try_exists()? && !self.overwrite {
            Err(Exception::IOError(format!("{} already exists", path.display())))
        } else {
            let mut node_public_key_file = File::create(path.to_path_buf())?;
            node_public_key_file.write_all(key_pair.node_public_key().as_bytes())?;
            Ok(path.to_path_buf())
        }
    }

    pub fn write_account_public_key(&self, key_pair: KeyPair) -> Result<PathBuf, Exception> {
        if !self.quiet {
            println!("{}: {}", &params.account_public_key_filename, key_pair.account_public_key());
        }
        let path = self.ensure_output_path_exists()?;
        let path = path.as_path().join(self.account_public_key());
        if path.try_exists()? && !self.overwrite {
            Err(Exception::IOError(format!("{} already exists", path.display())))
        } else {
            let mut account_public_key_file = File::create(path.to_path_buf())?;
            account_public_key_file.write_all(key_pair.account_public_key().as_bytes())?;
            Ok(path.to_path_buf())
        }
    }

    pub fn write_address(&self, key_pair: KeyPair) -> Result<PathBuf, Exception> {
        if !self.quiet {
            println!("{}: {:?}", &params.address_filename, key_pair.address());
        }
        let path = self.ensure_output_path_exists()?;
        let path = path.as_path().join(self.address());
        if path.try_exists()? && !self.overwrite {
            Err(Exception::IOError(format!("{} already exists", path.display())))
        } else {
            let mut address_file = File::create(path.to_path_buf())?;
            address_file.write_all(format!("{}", key_pair.address()).as_bytes())?;
            Ok(path.to_path_buf())
        }
    }
}


fn main() {
    let params = Params::parse();
    let key_pair = KeyPair::generate();

    if params.write {
        params.write_address(key_pair).expect("could not write address");
        params.write_account_public_key(key_pair).expect("could not write public key");
        params.write_node_secret_key(key_pair).expect("could not write secret key");
        params.write_node_public_key(key_pair).expect("could not write public key");
    }
}
