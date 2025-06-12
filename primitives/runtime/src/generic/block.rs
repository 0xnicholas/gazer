//*

use std::fmt;
use serde::{Serialize, Deserialize};

use crate::{
    codec::{Codec, Decode, Encode},
    traits::{
        self, Block as BlockT,  Header as HeaderT, MaybeSerialize, MaybeSerializeDeserialize,
        Member, NumberFor,
    },
    Justifications,
};
use alloc::vec::Vec;
// use sp_core::RuntimeDebug;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum BlockId<Block: BlockT> {
    Hash(Block::Hash),
    Number(NumberFor<Block>),
}

impl<Block: BlockT> BlockId<Block> {
    // Create a block ID from a hash.
    pub const fn hash(hash: Block::Hash) -> Self {
        BlockId::Hash(hash)
    }

    // Create a block ID from a number.
    pub const fn number(number: NumberFor<Block>) -> Self {
        BlockId::Number(number)
    }

    // Check if this block ID refers to the pre-genesis state.
    pub fn is_pre_genesis(&self) -> bool {
        match self {
            BlockId::Hash(hash) => hash == &Default::default(),
            BlockId::Number(_) => false,
        }
    }

    // Create a block ID for a pre-genesis state.
    pub fn pre_genesis() -> Self {
        BlockId::Hash(Default::default())
    }
}

impl<Block: BlockT> Copy for BlockId<Block> {}

impl<Block: BlockT> fmt::Display for BlockId<Block> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Abstraction over a gazer block.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Block<Header, Extrinsic>{
    pub Header,
    pub Extrinsic: Vec<Extrinsic>,
}

impl<Header, Extrinsic> traits::HeaderProvider for Block<Header, Extrinsic>
where
    Header: HeaderT,
{
    type HeaderT = Header;
}

impl<Header, Extrinsic> traits::Block for Block<Header, Extrinsic>
where
    Header: HeaderT + MaybeSerializeDeserialize,
    Extrinsic: Member + Codec + DecodeWithMemTracking + traits::ExtrinsicLike,
{
    type Extrinsic = Extrinsic;
    type Header = Header;
    type Hash = <Self::Header as traits::Header>::Hash;

    fn header(&self) -> &Self::Header {
        &self.header
    }
    fn extrinsics(&self) -> &[Self::Extrinsic] {
        &self.extrinsics[..]
    }
    fn deconstruct(self) -> (Self::Header, Vec<Self::Extrinsic>) {
        (self.header, self.extrinsics)
    }
    fn new(header: Self::Header, extrinsics: Vec<Self::Extrinsic>) -> Self {
        Block { header, extrinsics }
    }
    fn encode_from(header: &Self::Header, extrinsics: &[Self::Extrinsic]) -> Vec<u8> {
        (header, extrinsics).encode()
    }
}

/// Abstraction over a gazer block and justification.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct SignedBlock<Block> {
    /// Full block.
    pub block: Block,
    /// Block justification.
    pub justifications: Option<Justifications>,
}