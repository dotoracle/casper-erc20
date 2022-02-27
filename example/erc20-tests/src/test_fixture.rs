use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_erc20::constants as consts;
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, ContractHash, Key, PublicKey, SecretKey, RuntimeArgs, U256, U512,
};


const CONTRACT_ERC20_TOKEN: &str = "erc20_token.wasm";
const CONTRACT_KEY_NAME: &str = "erc20_token_contract";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestFixture {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
    pub minter: AccountHash,
    pub dev: AccountHash
}

impl TestFixture {
    pub const TOKEN_NAME: &'static str = "Test ERC20";
    pub const TOKEN_SYMBOL: &'static str = "TERC";
    pub const TOKEN_DECIMALS: u8 = 8;
    const TOKEN_TOTAL_SUPPLY_AS_U64: u64 = 1000;

    pub fn token_total_supply() -> U256 {
        Self::TOKEN_TOTAL_SUPPLY_AS_U64.into()
    }

    pub fn install_contract() -> TestFixture {
        let ali = PublicKey::from(&SecretKey::secp256k1_from_bytes([4u8; 32]).unwrap());
        let bob = PublicKey::from(&SecretKey::secp256k1_from_bytes([5u8; 32]).unwrap());
        let joe = PublicKey::from(&SecretKey::secp256k1_from_bytes([6u8; 32]).unwrap());
        let dev = PublicKey::from(&SecretKey::secp256k1_from_bytes([4u8; 32]).unwrap());
        let minter = PublicKey::from(&SecretKey::secp256k1_from_bytes([4u8; 32]).unwrap());
        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        
        let session_code = Code::from(CONTRACT_ERC20_TOKEN);
        let session_args = runtime_args! {
            consts::NAME_RUNTIME_ARG_NAME => TestFixture::TOKEN_NAME,
            consts::SYMBOL_RUNTIME_ARG_NAME => TestFixture::TOKEN_SYMBOL,
            consts::DECIMALS_RUNTIME_ARG_NAME => TestFixture::TOKEN_DECIMALS,
            consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => TestFixture::token_total_supply(),
            consts::MINTER_RUNTIME_ARG_NAME => Key::from(minter.to_account_hash()).to_formatted_string(),
            "swap_fee" => U256::zero(),
            "dev" => Key::from(dev.to_account_hash()).to_formatted_string(),
            "origin_chainid" => U256::one(),
            "origin_contract_address" => "ethereum contract address"
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        TestFixture {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
            minter: minter.to_account_hash(),
            dev: dev.to_account_hash()
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[CONTRACT_KEY_NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn token_name(&self) -> String {
        self.query_contract(consts::NAME_RUNTIME_ARG_NAME).unwrap()
    }

    pub fn dev(&self) -> String {
        self.query_contract("dev").unwrap()
    }

     pub fn swap_fee(&self) -> U256 {
        self.query_contract("swap_fee").unwrap()
    }

    pub fn token_symbol(&self) -> String {
        self.query_contract(consts::SYMBOL_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn token_decimals(&self) -> u8 {
        self.query_contract(consts::DECIMALS_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn read_request_map(&self, id: String) -> Option<U256> {
        let preimage = id.as_bytes();
        let item_key = hex::encode(&blake2b256(&preimage));

        let key = Key::Hash(self.contract_hash().value());
        println!("id {:?} {:?}", id, key);
        let value = self
            .context
            .query_dictionary_item(key, Some("request_map".to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn compute_request_map(&self, id: String) -> String {
        let preimage = id.as_bytes();
        hex::encode(&blake2b256(&preimage))
    }
    pub fn get_minter(&self) -> String {
        self.query_contract(consts::MINTER_RUNTIME_ARG_NAME).unwrap()
    }

    pub fn total_supply(&self) -> U256 {
        self.query_contract(consts::TOTAL_SUPPLY_KEY_NAME).unwrap()
    }

    pub fn change_minter(&mut self, new_minter: Key, sender: Sender) {
        self.call(
            sender,
            consts::CHANGE_MINTER_ENTRY_POINT_NAME,
            runtime_args! {
                consts::MINTER_RUNTIME_ARG_NAME => new_minter
            },
        );
    }

    pub fn request_bridge_back(&mut self, amount: U256, fee: U256, to_chainid: U256, receiver_address: String, id: String, sender: Sender) {
        self.call(
            sender,
            "request_bridge_back",
            runtime_args! {
                consts::AMOUNT_RUNTIME_ARG_NAME => amount,
                "fee" => fee,
                "to_chainid" => to_chainid,
                "receiver_address" => receiver_address,
                "id" => id
            },
        );
    }

    pub fn change_dev(&mut self, new_dev: Key, sender: Sender) {
        self.call(
            sender,
            "change_dev",
            runtime_args! {
                "dev" => new_dev
            },
        );
    }

    pub fn change_swap_fee(&mut self, new_swap_fee: U256, sender: Sender) {
        self.call(
            sender,
            "change_swap_fee",
            runtime_args! {
                "swap_fee" => new_swap_fee
            },
        );
    }

    pub fn allowance(&self, owner: Key, spender: Key) -> Option<U256> {
        let mut preimage = Vec::new();
        preimage.append(&mut owner.to_bytes().unwrap());
        preimage.append(&mut spender.to_bytes().unwrap());
        let key_bytes = blake2b256(&preimage);
        let allowance_item_key = hex::encode(&key_bytes);

        let key = Key::Hash(self.contract_hash().value());

        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(consts::ALLOWANCES_KEY_NAME.to_string()),
                allowance_item_key,
            )
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn transfer(&mut self, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn mint(&mut self, recipient: Key, amount: U256, swap_fee: U256, mintid: String, sender: Sender) {
        self.call(
            sender,
            consts::MINT_ENTRY_POINT_NAME,
            runtime_args! {
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount,
                "swap_fee" => swap_fee,
                "mintid" => mintid
            },
        );
    }

    pub fn burn(&mut self, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::BURN_ENTRY_POINT_NAME,
            runtime_args! {
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn approve(&mut self, spender: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::APPROVE_ENTRY_POINT_NAME,
            runtime_args! {
                consts::SPENDER_RUNTIME_ARG_NAME => spender,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                consts::OWNER_RUNTIME_ARG_NAME => owner,
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }
}
