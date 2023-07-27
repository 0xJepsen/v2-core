use arbiter::agent::simple_arbitrageur::SimpleArbitrageur;
use arbiter::agent::{Agent, AgentType, SimulationEventFilter};
use arbiter::{
    environment::contract::SimulationContract,
    manager,
    utils::{float_to_wad, recast_address, unpack_execution},
};
// dynamic imports... generate with build.sh
use bindings::{uniswap_v2_factory, uniswap_v2_pair};
use ethers::{
    abi::{encode_packed, Token, Tokenize},
    prelude::{Address, U128, U256},
    types::H160,
};
use revm::primitives::B160;


pub fn run(manager: &mut manager::SimulationManager) -> Result<(), Box<dyn std::error::Error>> {
    let admin = manager.agents.get("admin").unwrap();

    // Deploy weth
    let uniswap_v2_factory = SimulationContract::new(
        uniswap_v2_factory::UNISWAPV2FACTORY_ABI.clone(),
        uniswap_v2_factory::UNISWAPV2FACTORY_BYTECODE.clone(),
    );
    let (uniswap_v2_factory_contract, result) = admin.deploy(uniswap_v2_factory, (recast_address(admin.address()).into_tokens()))?;
    assert!(result.is_success());
    Ok(())
}