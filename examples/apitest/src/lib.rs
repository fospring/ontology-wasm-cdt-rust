#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;
use ostd::abi::Dispatcher;
use ostd::abi::{Decoder, Sink, Source};
use ostd::contract::ont;
use ostd::prelude::*;
use ostd::str;
use ostd::types::to_neo_bytes;
use ostd::{console, runtime};

#[ostd::abi_codegen::contract]
pub trait ApiTest {
    fn timestamp(&self) -> u64;
    fn block_height(&self) -> u32;
    fn self_address(&self) -> Address;
    fn caller_address(&self) -> Address;
    fn entry_address(&self) -> Address;
    fn contract_debug(&self, content: &str) -> ();
    //    fn contract_delete(&self) -> ();
    fn check_witness(&self, addr: &Address) -> bool;
    fn get_current_blockhash(&self) -> H256;
    fn get_current_txhash(&self) -> H256;
    fn call_wasm_name(&self, contract_address: &Address) -> String;
    fn call_wasm_balance_of(&self, contract_address: &Address, addr: &Address) -> U256;
    fn call_wasm_transfer(
        &self, contract_address: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool;
    fn call_neovm_transfer(
        &self, contract_address: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool;
    fn call_ont_transfer(&self, from: &Address, to: &Address, amount: U256) -> bool;
    fn call_ont_balance_of(&self, address: &Address) -> U256;
    fn call_ont_approve(&self, from: &Address, to: &Address, amount: U256) -> bool;
    fn call_ont_allowance(&self, from: &Address, to: &Address) -> U256;
    fn call_ont_transfer_from(
        &self, sender: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool;
    fn contract_migrate(
        &self, code: Vec<u8>, vm_type: u32, name: &str, version: &str, author: &str, email: &str,
        desc: &str,
    ) -> bool;
}

pub(crate) struct ApiTestInstance;

impl ApiTest for ApiTestInstance {
    fn timestamp(&self) -> u64 {
        runtime::timestamp()
    }
    fn block_height(&self) -> u32 {
        runtime::block_height()
    }
    fn self_address(&self) -> Address {
        runtime::address()
    }
    fn caller_address(&self) -> Address {
        runtime::caller()
    }
    fn entry_address(&self) -> Address {
        runtime::entry_address()
    }
    fn contract_debug(&self, content: &str) {
        console::debug(content);
    }
    //    fn contract_delete(&self) -> () {
    //        runtime::contract_delete();
    //    }
    fn check_witness(&self, addr: &Address) -> bool {
        let b = runtime::check_witness(addr);
        if b {
            runtime::notify("success".as_bytes());
            true
        } else {
            runtime::notify("failed".as_bytes());
            false
        }
    }
    fn get_current_blockhash(&self) -> H256 {
        runtime::current_blockhash()
    }
    fn get_current_txhash(&self) -> H256 {
        runtime::current_txhash()
    }
    fn call_wasm_name(&self, contract_address: &Address) -> String {
        let mut sink = Sink::new(16);
        sink.write("name".to_string());
        console::debug(&format!("{:?}", contract_address));
        let res = runtime::call_contract(contract_address, sink.bytes()).unwrap();
        let s = str::from_utf8(res.as_slice()).unwrap();
        console::debug(s);
        let mut source = Source::new(res);
        source.read().unwrap()
    }
    fn call_wasm_balance_of(&self, contract_address: &Address, addr: &Address) -> U256 {
        let mut sink = Sink::new(16);
        sink.write(("balance_of".to_string(), addr));
        let res = runtime::call_contract(contract_address, sink.bytes());
        if res.is_some() {
            let temp = res.unwrap();
            let mut source = Source::new(temp);
            U256::decode(&mut source).unwrap()
        } else {
            U256::zero()
        }
    }
    fn call_wasm_transfer(
        &self, contract_address: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write(("transfer".to_string(), from, to, amount));
        let res = runtime::call_contract(contract_address, sink.bytes());
        if res.is_some() {
            true
        } else {
            false
        }
    }
    fn call_neovm_transfer(
        &self, contract_address: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write(to_neo_bytes(amount));
        sink.write_neovm_address(to);
        sink.write_neovm_address(from);
        sink.write(83u8);
        sink.write(193u8);
        sink.write("transfer".to_string());
        sink.write(103u8);
        sink.write(contract_address);
        let res = runtime::call_contract(contract_address, sink.bytes());
        if res.is_some() {
            let data = res.unwrap();
            runtime::notify("true".as_bytes());
            let s = str::from_utf8(data.as_slice()).unwrap();
            console::debug(s);
            return true;
        } else {
            false
        }
    }
    fn call_ont_transfer(&self, from: &Address, to: &Address, amount: U256) -> bool {
        let state = ont::State { from: from.clone(), to: to.clone(), amount: amount };
        ont::transfer(&[state])
    }
    fn call_ont_approve(&self, from: &Address, to: &Address, amount: U256) -> bool {
        ont::approve(from, to, amount)
    }
    fn call_ont_allowance(&self, from: &Address, to: &Address) -> U256 {
        ont::allowance(from, to)
    }
    fn call_ont_balance_of(&self, address: &Address) -> U256 {
        ont::balance_of(address)
    }
    fn call_ont_transfer_from(
        &self, sender: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool {
        ont::transfer_from(sender, from, to, amount)
    }
    fn contract_migrate(
        &self, code: Vec<u8>, vm_type: u32, name: &str, version: &str, author: &str, email: &str,
        desc: &str,
    ) -> bool {
        runtime::contract_migrate(code.as_slice(), vm_type, name, version, author, email, desc);
        true
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher = ApiTestDispatcher::new(ApiTestInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;
