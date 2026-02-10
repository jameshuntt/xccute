use std::path::PathBuf;

use crate::ShellCommand;

#[derive(Debug)]
pub enum KmsCommand {
    SoftHsm2(SoftHsm2Command),
    // Extendable to other backends like AwsKms, YubiHsm, etc.
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
    InitToken(TokenInitArgs),
    DeleteToken(TokenDeleteArgs),
    ChangePin(TokenChangePinArgs),
    ListSlots,
    ShowSlotInfo(TokenSlotInfoArgs),
}

#[derive(Debug)]
pub struct TokenInitArgs {
    pub slot: u32,
    pub label: String,
    pub pin: String,
    pub so_pin: String,
    pub module: Option<PathBuf>,
}

#[derive(Debug)]
pub struct TokenDeleteArgs {
    pub slot: u32,
}

#[derive(Debug)]
pub struct TokenChangePinArgs {
    pub slot: u32,
    pub old_pin: String,
    pub new_pin: String,
}

#[derive(Debug)]
pub struct TokenSlotInfoArgs {
    pub slot: u32,
}

#[derive(Debug)]
pub enum KeyCommand {
    KeyGen(KeyGenArgs),
    ImportKey(ImportKeyArgs),
    ExportPubKey(ExportPubKeyArgs),
    DeleteKey(DeleteKeyArgs),
    ListKeys(ListKeysArgs),
    ShowAttributes(ShowAttributesArgs),
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

#[derive(Debug)]
pub struct ImportKeyArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
    pub file: PathBuf,
}

#[derive(Debug)]
pub struct ExportPubKeyArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
    pub output: PathBuf,
}

#[derive(Debug)]
pub struct DeleteKeyArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
}

#[derive(Debug)]
pub struct ListKeysArgs {
    pub slot: u32,
    pub pin: String,
    pub module: Option<PathBuf>,
}

#[derive(Debug)]
pub struct ShowAttributesArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
}

#[derive(Debug)]
pub enum KeyType {
    Rsa { bits: u32 },
    Ec { curve: String },
    Aes { size: u32 },
}

#[derive(Debug)]
pub enum CertCommand {
    ImportCert(ImportCertArgs),
    ExportCert(ExportCertArgs),
    LinkCertToKey(LinkCertToKeyArgs),
}

#[derive(Debug)]
pub struct ImportCertArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
    pub file: PathBuf,
}

#[derive(Debug)]
pub struct ExportCertArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
    pub output: PathBuf,
}

#[derive(Debug)]
pub struct LinkCertToKeyArgs {
    pub slot: u32,
    pub pin: String,
    pub key_label: String,
    pub cert_label: String,
}

#[derive(Debug)]
pub enum CryptoCommand {
    Sign(SignArgs),
    Verify(VerifyArgs),
    Digest(DigestArgs),
    Encrypt(EncryptArgs),
    Decrypt(DecryptArgs),
    Derive(DeriveArgs),
}

#[derive(Debug)]
pub struct SignArgs {
    pub slot: u32,
    pub pin: String,
    pub key_label: String,
    pub input: PathBuf,
    pub output: PathBuf,
    pub mechanism: String,
}

#[derive(Debug)]
pub struct VerifyArgs {
    pub slot: u32,
    pub pin: String,
    pub key_label: String,
    pub signature: PathBuf,
    pub input: PathBuf,
}

#[derive(Debug)]
pub struct DigestArgs {
    pub slot: u32,
    pub pin: String,
    pub input: PathBuf,
    pub output: PathBuf,
    pub mechanism: String,
}

#[derive(Debug)]
pub struct EncryptArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
    pub input: PathBuf,
    pub output: PathBuf,
}

#[derive(Debug)]
pub struct DecryptArgs {
    pub slot: u32,
    pub pin: String,
    pub label: String,
    pub input: PathBuf,
    pub output: PathBuf,
}

#[derive(Debug)]
pub struct DeriveArgs {
    pub slot: u32,
    pub pin: String,
    pub base_key_label: String,
    pub derived_key_label: String,
    pub mechanism: String,
}

#[derive(Debug)]
pub enum UtilityCommand {
    ListMechanisms(ListMechanismsArgs),
    Benchmark(BenchmarkArgs),
    TokenDbPath,
    CapabilitiesSummary(CapabilitiesSummaryArgs),
}

#[derive(Debug)]
pub struct ListMechanismsArgs {
    pub slot: u32,
}

#[derive(Debug)]
pub struct BenchmarkArgs {
    pub slot: u32,
    pub pin: String,
}

#[derive(Debug)]
pub struct CapabilitiesSummaryArgs {
    pub slot: u32,
}

impl ShellCommand for KmsCommand {
    fn build(&self) -> String {
        match self {
            KmsCommand::SoftHsm2(inner) => inner.build(),
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
