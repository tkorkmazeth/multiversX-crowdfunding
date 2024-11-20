#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]

pub trait Crowdfunding {
    #[init]
    fn init(&self) {}
}
