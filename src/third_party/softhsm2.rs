use std::path::PathBuf;

use crate::ShellCommand;

#[derive(Debug)]
pub enum KmsCommand {
    SoftHsm2(SoftHsm2Command),
    // In future: add HardwareHsm, AwsKms, etc.
}

#[derive(Debug)]
pub enum SoftHsm2Command {
    Token(TokenCommand),
    Key(KeyCommand),
    Cert(CertCommand),
    Crypto(CryptoCommand),
    Utility(UtilityCommand),
}

#[derive(Debug)]
pub enum TokenCommand {
    InitToken { slot: u32, label: String, pin: String, so_pin: String, module: Option<PathBuf>, },
    DeleteToken { slot: u32, },
    ChangePin { slot: u32, old_pin: String, new_pin: String, },
    ListSlots,
    ShowSlotInfo { slot: u32, },
}

#[derive(Debug)]
pub enum KeyCommand {
    KeyGen(KeyGenArgs),
    ImportKey { slot: u32, pin: String, label: String, file: PathBuf, },
    ExportPubKey { slot: u32, pin: String, label: String, output: PathBuf, },
    DeleteKey { slot: u32, pin: String, label: String, },
    ListKeys { slot: u32, pin: String, module: Option<PathBuf>, },
    ShowAttributes { slot: u32, pin: String, label: String, },
}

#[derive(Debug)]
pub enum CertCommand {
    ImportCert { slot: u32, pin: String, label: String, file: PathBuf, },
    ExportCert { slot: u32, pin: String, label: String, output: PathBuf, },
    LinkCertToKey { slot: u32, pin: String, key_label: String, cert_label: String, },
}

#[derive(Debug)]
pub enum CryptoCommand {
    SignData { slot: u32, pin: String, key_label: String, input: PathBuf, output: PathBuf, mechanism: String, },
    VerifyData { slot: u32, pin: String, key_label: String, signature: PathBuf, input: PathBuf, },
    Digest { slot: u32, pin: String, input: PathBuf, output: PathBuf, mechanism: String, },
    Encrypt { slot: u32, pin: String, label: String, input: PathBuf, output: PathBuf, },
    Decrypt { slot: u32, pin: String, label: String, input: PathBuf, output: PathBuf, },
    DeriveKey { slot: u32, pin: String, base_key_label: String, derived_key_label: String, mechanism: String, },
}

#[derive(Debug)]
pub enum UtilityCommand {
    ListMechanisms { slot: u32, },
    Benchmark { slot: u32, pin: String, },
    TokenDbPath,
    CapabilitiesSummary { slot: u32, },
}

#[derive(Debug)]
pub enum KeyType {
    Rsa { bits: u32 },
    Ec { curve: String },
    Aes { size: u32 },
}

#[derive(Debug)]
pub struct KeyGenArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
    pub key_type: KeyType,
    pub module: Option<PathBuf>,
    pub login: bool,
}

impl ShellCommand for KmsCommand {
    fn build(&self) -> String {
        match self {
            KmsCommand::SoftHsm2(inner) => match inner {
                SoftHsm2Command::Token(cmd) => format!("token: {:?}", cmd),
                SoftHsm2Command::Key(cmd) => format!("key: {:?}", cmd),
                SoftHsm2Command::Cert(cmd) => format!("cert: {:?}", cmd),
                SoftHsm2Command::Crypto(cmd) => format!("crypto: {:?}", cmd),
                SoftHsm2Command::Utility(cmd) => format!("util: {:?}", cmd),
            },
        }
    }
}

impl ShellCommand for SoftHsm2Command {
    fn build(&self) -> String {
        match self {
            SoftHsm2Command::Token(cmd) => format!("token: {:?}", cmd),
            SoftHsm2Command::Key(cmd) => format!("key: {:?}", cmd),
            SoftHsm2Command::Cert(cmd) => format!("cert: {:?}", cmd),
            SoftHsm2Command::Crypto(cmd) => format!("crypto: {:?}", cmd),
            SoftHsm2Command::Utility(cmd) => format!("util: {:?}", cmd),
        }
    }
}
