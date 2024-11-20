use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/crowdfunding.mxsc.json", crowdfunding::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/crowdfunding.scen.json");
}
