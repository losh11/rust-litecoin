// Written in 2014 by Andrew Poelstra <apoelstra@wpsoftware.net>
// SPDX-License-Identifier: CC0-1.0

//! Blockdata constants.
//!
//! This module provides various constants relating to the blockchain and
//! consensus code. In particular, it defines the genesis block and its
//! single transaction.
//!

use crate::prelude::*;

use core::default::Default;

use crate::hashes::hex::{self, HexIterator};
use crate::hashes::{Hash, sha256d};
use crate::blockdata::opcodes;
use crate::blockdata::script;
use crate::blockdata::locktime::PackedLockTime;
use crate::blockdata::transaction::{OutPoint, Transaction, TxOut, TxIn, Sequence};
use crate::blockdata::block::{Block, BlockHeader};
use crate::blockdata::witness::Witness;
use crate::network::constants::Network;
use crate::util::uint::Uint256;
use crate::internal_macros::{impl_array_newtype, impl_bytes_newtype};

/// How many satoshis are in "one bitcoin"
pub const COIN_VALUE: u64 = 100_000_000;
/// How many seconds between blocks we expect on average
pub const TARGET_BLOCK_SPACING: u32 = 150;
/// How many blocks between diffchanges
pub const DIFFCHANGE_INTERVAL: u32 = 2016;
/// How much time on average should occur between diffchanges
pub const DIFFCHANGE_TIMESPAN: u32 = 150 * 2016;
/// The maximum allowed weight for a block, see BIP 141 (network rule)
pub const MAX_BLOCK_WEIGHT: u32 = 4_000_000;
/// The minimum transaction weight for a valid serialized transaction
pub const MIN_TRANSACTION_WEIGHT: u32 = 4 * 60;
/// The factor that non-witness serialization data is multiplied by during weight calculation
pub const WITNESS_SCALE_FACTOR: usize = 4;
/// The maximum allowed number of signature check operations in a block
pub const MAX_BLOCK_SIGOPS_COST: i64 = 80_000;
/// Mainnet (bitcoin) pubkey address prefix.
pub const PUBKEY_ADDRESS_PREFIX_MAIN: u8 = 48; // 0x30
/// Mainnet (bitcoin) script address prefix.
pub const SCRIPT_ADDRESS_PREFIX_MAIN: u8 = 50; // 0x32
/// Test (testnet, signet, regtest) pubkey address prefix.
pub const PUBKEY_ADDRESS_PREFIX_TEST: u8 = 111; // 0x6f
/// Test (testnet, signet, regtest) script address prefix.
pub const SCRIPT_ADDRESS_PREFIX_TEST: u8 = 58; // 0x3a
/// The maximum allowed script size.
pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;
/// How may blocks between halvings.
pub const SUBSIDY_HALVING_INTERVAL: u32 = 840_000;
/// Maximum allowed value for an integer in Script.
pub const MAX_SCRIPTNUM_VALUE: u32 = 0x80000000; // 2^31

/// In Bitcoind this is insanely described as ~((u256)0 >> 32)
pub fn max_target(_: Network) -> Uint256 {
    Uint256::from_u64(0xFFFF).unwrap() << 208
}

/// The maximum value allowed in an output (useful for sanity checking,
/// since keeping everything below this value should prevent overflows
/// if you are doing anything remotely sane with monetary values).
pub fn max_money(_: Network) -> u64 {
    84_000_000 * COIN_VALUE
}

/// Constructs and returns the coinbase (and only) transaction of the Bitcoin genesis block
fn bitcoin_genesis_tx() -> Transaction {
    // Base
    let mut ret = Transaction {
        version: 1,
        lock_time: PackedLockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    // Inputs
    let in_script = script::Builder::new().push_scriptint(486604799)
                                          .push_scriptint(4)
                                          .push_slice("NY Times 05/Oct/2011 Steve Jobs, Apple’s Visionary, Dies at 56".as_bytes())
                                          .into_script();
    ret.input.push(TxIn {
        previous_output: OutPoint::null(),
        script_sig: in_script,
        sequence: Sequence::MAX,
        witness: Witness::default(),
    });

    // Outputs
    let script_bytes: Result<Vec<u8>, hex::Error> =
        HexIterator::new("040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9").unwrap()
            .collect();
    let out_script = script::Builder::new()
        .push_slice(script_bytes.unwrap().as_slice())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script();
    ret.output.push(TxOut {
        value: 50 * COIN_VALUE,
        script_pubkey: out_script
    });

    // end
    ret
}

/// Constructs and returns the genesis block
pub fn genesis_block(network: Network) -> Block {
    let txdata = vec![bitcoin_genesis_tx()];
    let hash: sha256d::Hash = txdata[0].txid().into();
    let merkle_root = hash.into();
    match network {
        Network::Bitcoin => {
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1317972665,
                    bits: 0x1e0ffff0,
                    nonce: 2084524493
                },
                txdata,
            }
        }
        Network::Testnet => {
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1486949366,
                    bits: 0x1e0ffff0,
                    nonce: 293345
                },
                txdata,
            }
        }
        Network::Signet => {
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1598918400,
                    bits: 0x1e0377ae,
                    nonce: 52613770
                },
                txdata,
            }
        }
        Network::Regtest => {
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Hash::all_zeros(),
                    merkle_root,
                    time: 1296688602,
                    bits: 0x207fffff,
                    nonce: 2
                },
                txdata,
            }
        }
    }
}

// Mainnet value can be verified at https://github.com/lightning/bolts/blob/master/00-introduction.md
const GENESIS_BLOCK_HASH_BITCOIN: [u8; 32] = [226, 191, 4, 126, 126, 90, 25, 26, 164, 239, 52, 211, 20, 151, 157, 201, 152, 110, 15, 25, 37, 30, 218, 186, 89, 64, 253, 31, 227, 101, 167, 18];
const GENESIS_BLOCK_HASH_TESTNET: [u8; 32] = [160, 41, 62, 78, 235, 61, 166, 230, 245, 111, 129, 237, 89, 95, 87, 136, 13, 26, 33, 86, 158, 19, 238, 253, 217, 81, 40, 75, 90, 98, 102, 73];
const GENESIS_BLOCK_HASH_SIGNET: [u8; 32] = [246, 30, 238, 59, 99, 163, 128, 164, 119, 160, 99, 175, 50, 178, 187, 201, 124, 159, 249, 240, 31, 44, 66, 37, 233, 115, 152, 129, 8, 0, 0, 0];
const GENESIS_BLOCK_HASH_REGTEST: [u8; 32] = [6, 34, 110, 70, 17, 26, 11, 89, 202, 175, 18, 96, 67, 235, 91, 191, 40, 195, 79, 58, 94, 51, 42, 31, 199, 178, 183, 60, 241, 136, 145, 15];

/// The uniquely identifying hash of the target blockchain.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChainHash([u8; 32]);
impl_array_newtype!(ChainHash, u8, 32);
impl_bytes_newtype!(ChainHash, 32);

impl ChainHash {
    /// Returns the hash of the `network` genesis block for use as a chain hash.
    ///
    /// See [BOLT 0](https://github.com/lightning/bolts/blob/ffeece3dab1c52efdb9b53ae476539320fa44938/00-introduction.md#chain_hash)
    /// for specification.
    pub fn using_genesis_block(network: Network) -> Self {
        match network {
            Network::Bitcoin => ChainHash(GENESIS_BLOCK_HASH_BITCOIN),
            Network::Testnet => ChainHash(GENESIS_BLOCK_HASH_TESTNET),
            Network::Signet => ChainHash(GENESIS_BLOCK_HASH_SIGNET),
            Network::Regtest => ChainHash(GENESIS_BLOCK_HASH_REGTEST),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hashes::hex::{ToHex, FromHex};
    use crate::network::constants::Network;
    use crate::consensus::encode::serialize;
    use crate::blockdata::locktime::PackedLockTime;

    #[test]
    fn bitcoin_genesis_first_transaction() {
        let gen = bitcoin_genesis_tx();

        assert_eq!(gen.version, 1);
        assert_eq!(gen.input.len(), 1);
        assert_eq!(gen.input[0].previous_output.txid, Hash::all_zeros());
        assert_eq!(gen.input[0].previous_output.vout, 0xFFFFFFFF);
        assert_eq!(serialize(&gen.input[0].script_sig),
                   Vec::from_hex("4804ffff001d0104404e592054696d65732030352f4f63742f32303131205374657665204a6f62732c204170706c65e280997320566973696f6e6172792c2044696573206174203536").unwrap());

        assert_eq!(gen.input[0].sequence, Sequence::MAX);
        assert_eq!(gen.output.len(), 1);
        assert_eq!(serialize(&gen.output[0].script_pubkey),
                   Vec::from_hex("4341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac").unwrap());
        assert_eq!(gen.output[0].value, 50 * COIN_VALUE);
        assert_eq!(gen.lock_time, PackedLockTime::ZERO);

        assert_eq!(gen.wtxid().to_hex(), "97ddfbbae6be97fd6cdf3e7ca13232a3afff2353e29badfab7f73011edd4ced9");
    }

    #[test]
    fn bitcoin_genesis_full_block() {
        let gen = genesis_block(Network::Bitcoin);

        assert_eq!(gen.header.version, 1);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(gen.header.merkle_root.to_hex(), "97ddfbbae6be97fd6cdf3e7ca13232a3afff2353e29badfab7f73011edd4ced9");

        assert_eq!(gen.header.time, 1317972665);
        assert_eq!(gen.header.bits, 0x1e0ffff0);
        assert_eq!(gen.header.nonce, 2084524493);
        assert_eq!(gen.header.block_hash().to_hex(), "12a765e31ffd4059bada1e25190f6e98c99d9714d334efa41a195a7e7e04bfe2");
    }

    #[test]
    fn testnet_genesis_full_block() {
        let gen = genesis_block(Network::Testnet);
        assert_eq!(gen.header.version, 1);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(gen.header.merkle_root.to_hex(), "97ddfbbae6be97fd6cdf3e7ca13232a3afff2353e29badfab7f73011edd4ced9");
        assert_eq!(gen.header.time, 1486949366);
        assert_eq!(gen.header.bits, 0x1e0ffff0);
        assert_eq!(gen.header.nonce, 293345);
        assert_eq!(gen.header.block_hash().to_hex(), "4966625a4b2851d9fdee139e56211a0d88575f59ed816ff5e6a63deb4e3e29a0");
    }

    #[test]
    fn signet_genesis_full_block() {
        let gen = genesis_block(Network::Signet);
        assert_eq!(gen.header.version, 1);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(gen.header.merkle_root.to_hex(), "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b");
        assert_eq!(gen.header.time, 1598918400);
        assert_eq!(gen.header.bits, 0x1e0377ae);
        assert_eq!(gen.header.nonce, 52613770);
        assert_eq!(gen.header.block_hash().to_hex(), "00000008819873e925422c1ff0f99f7cc9bbb232af63a077a480a3633bee1ef6");
    }

    // The *_chain_hash tests are sanity/regression tests, they verify that the const byte array
    // representing the genesis block is the same as that created by hashing the genesis block.
    fn chain_hash_and_genesis_block(network: Network) {
        use crate::hashes::sha256;

        // The genesis block hash is a double-sha256 and it is displayed backwards.
        let genesis_hash = genesis_block(network).block_hash();
        // We abuse the sha256 hash here so we get a LowerHex impl that does not print the hex backwards.
        let hash = sha256::Hash::from_slice(&genesis_hash.into_inner()).unwrap();
        let want = format!("{:02x}", hash);

        let chain_hash = ChainHash::using_genesis_block(network);
        let got = format!("{:02x}", chain_hash);

        // Compare strings because the spec specifically states how the chain hash must encode to hex.
        assert_eq!(got, want);
    }

    macro_rules! chain_hash_genesis_block {
        ($($test_name:ident, $network:expr);* $(;)*) => {
            $(
                #[test]
                fn $test_name() {
                    chain_hash_and_genesis_block($network);
                }
            )*
        }
    }

    chain_hash_genesis_block! {
        mainnet_chain_hash_genesis_block, Network::Bitcoin;
        testnet_chain_hash_genesis_block, Network::Testnet;
        signet_chain_hash_genesis_block, Network::Signet;
        regtest_chain_hash_genesis_block, Network::Regtest;
    }

    // Test vector taken from: https://github.com/lightning/bolts/blob/master/00-introduction.md
    #[test]
    fn mainnet_chain_hash_test_vector() {
        let got = ChainHash::using_genesis_block(Network::Bitcoin).to_hex();
        let want = "e2bf047e7e5a191aa4ef34d314979dc9986e0f19251edaba5940fd1fe365a712";
        assert_eq!(got, want);
    }
}

