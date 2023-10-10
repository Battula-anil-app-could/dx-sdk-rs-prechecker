use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn dct_system_sc_go() {
    world().run("scenarios/dct_system_sc.scen.json");
}
