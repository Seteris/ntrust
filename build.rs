use std::collections::HashMap;

fn main() {
    let mut features = HashMap::new();
    features.insert("ntruhps2048509", cfg!(feature = "ntruhps2048509"));
    features.insert("ntruhps2048677", cfg!(feature = "ntruhps2048677"));
    features.insert("ntruhps4096821", cfg!(feature = "ntruhps4096821"));
    features.insert("ntruhrss701", cfg!(feature = "ntruhrss701"));

    let mut target_feature = "";
    for (feature, in_use) in features {
        if target_feature != "" && in_use {
            panic!("[ERROR] Configuration error: \n\t{} and {} cannot be used simultaneously!\n\tPlease select only one feature.", target_feature, feature);
        } else if in_use {
            target_feature = feature;
        }
    }

    if target_feature == ""
    {
        panic!("Configuration error: No features selected. Please specify a feature for this crate.");
    }
}