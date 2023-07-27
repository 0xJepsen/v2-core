use arbiter::agent::simple_arbitrageur::SimpleArbitrageur;
use arbiter::agent::{Agent, AgentType, SimulationEventFilter};
use arbiter::{
    environment::contract::SimulationContract,
    manager,
    utils::{float_to_wad, recast_address, unpack_execution},
};
// dynamic imports... generate with build.sh
use bindings::{uniswap_v2_factory, uniswap_v2_pair, uniswap_v2_router02, weth};
use ethers::{
    abi::{encode_packed, Token, Tokenize},
    prelude::{Address, U128, U256},
    types::H160,
};
use revm::primitives::B160;

use super::common;

pub fn run(manager: &mut manager::SimulationManager) -> Result<(), Box<dyn std::error::Error>> {
    let admin = manager.agents.get("admin").unwrap();

    // Deploy weth
    let weth = SimulationContract::new(weth::WETH_ABI.clone(), weth::WETH_BYTECODE.clone());
    let (weth_contract, _result) = admin.deploy(weth, vec![])?;

    let uniswap_v2_factory = SimulationContract::new(
        uniswap_v2_factory::UNISWAP_V2_FACTORY_ABI.clone(),
        uniswap_v2_factory::UNISWAP_V2_FACTORY_BYTECODE.clone(),
    );
    let (uniswap_v2_factory_contract, _result) = admin.deploy(uniswap_v2_factory, vec![])?;
}