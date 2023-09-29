// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchanges {
    #[prost(message, repeated, tag="1")]
    pub exchanges: ::prost::alloc::vec::Vec<Exchange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchange {
    #[prost(message, optional, tag="1")]
    pub buyer: ::core::option::Option<Account>,
    #[prost(string, tag="2")]
    pub sold_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tokens_sold: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub bought_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub tokens_bought: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub block_number: u64,
    #[prost(uint64, tag="12")]
    pub timestamp: u64,
    #[prost(uint32, tag="13")]
    pub log_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address_token_one: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub address_token_two: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub address_token_three: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
