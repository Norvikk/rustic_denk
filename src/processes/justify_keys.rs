use crate::actions::read;
use crate::ProcessConfig;

pub fn determine_keys(config: &mut ProcessConfig) {
    read::files(config);
    assign_synapse(config);
    justify_soft(config);
}

fn justify_soft(config: &mut ProcessConfig) {
    let split_entries: Vec<&str> = config.read_keys.split(&config.system_synapse).collect();

    for i in (0..split_entries.len() - 1).step_by(2) {
        config.process_soft_bundle.insert(
            split_entries[i + 1].to_string(),
            split_entries[i].to_string(),
        );
    }
}

fn assign_synapse(config: &mut ProcessConfig) {
    let count_of_steps = 10;

    config.system_synapse = config.read_keys[1..count_of_steps + 1].to_string();
}
