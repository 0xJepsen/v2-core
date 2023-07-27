use arbiter::stochastic::price_process::{PriceProcess, PriceProcessType, OU};
use arbiter::utils::wad_to_float;
use arbiter::{
    agent::{Agent, AgentType},
    manager::SimulationManager,
    utils::recast_address,
};
use ethers::abi::Tokenize;
use visualize::{design::*, plot::*};

mod setup;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut manager = SimulationManager::new();

    setup::run(&mut manager)?;
    println!("Hello, world!");
    Ok(())
}