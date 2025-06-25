use once_cell::sync::Lazy;
use rand::{Rng, thread_rng};
use std::collections::HashSet;
use std::sync::Mutex;

static USED_NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn generate_unique_name_locked(used_names: &mut HashSet<String>) -> String {
    let mut rng = thread_rng();
    loop {
        let name = format!(
            "{}{}{:03}",
            rng.gen_range('A'..='Z'),
            rng.gen_range('A'..='Z'),
            rng.gen_range(0..=999)
        );
        if used_names.insert(name.clone()) {
            return name;
        }
    }
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut used_names = USED_NAMES.lock().unwrap();
        Robot {
            name: generate_unique_name_locked(&mut used_names),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut used_names = USED_NAMES.lock().unwrap();
        used_names.remove(&self.name);
        self.name = generate_unique_name_locked(&mut used_names);
    }
}
