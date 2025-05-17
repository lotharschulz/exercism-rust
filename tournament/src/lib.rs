use std::collections::HashMap;

#[derive(Default)]
struct TeamStats {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

pub fn tally(match_results: &str) -> String {
    let mut table: HashMap<String, TeamStats> = HashMap::new();
    for line in match_results.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue;
        }
        let (team1, team2, result) = (parts[0], parts[1], parts[2]);
        // Avoid double mutable borrow by splitting updates
        {
            let entry = table.entry(team1.to_string()).or_default();
            entry.mp += 1;
        }
        {
            let entry = table.entry(team2.to_string()).or_default();
            entry.mp += 1;
        }
        match result {
            "win" => {
                let entry = table.get_mut(team1).unwrap();
                entry.w += 1;
                entry.p += 3;
                let entry = table.get_mut(team2).unwrap();
                entry.l += 1;
            }
            "loss" => {
                let entry = table.get_mut(team2).unwrap();
                entry.w += 1;
                entry.p += 3;
                let entry = table.get_mut(team1).unwrap();
                entry.l += 1;
            }
            "draw" => {
                let entry = table.get_mut(team1).unwrap();
                entry.d += 1;
                entry.p += 1;
                let entry = table.get_mut(team2).unwrap();
                entry.d += 1;
                entry.p += 1;
            }
            _ => {}
        }
    }
    let mut teams: Vec<_> = table.into_iter().collect();
    teams.sort_by(|a, b| {
        b.1.p.cmp(&a.1.p).then_with(|| a.0.cmp(&b.0))
    });
    let mut output = String::from("Team                           | MP |  W |  D |  L |  P");
    for (name, stats) in teams {
        output.push('\n');
        output.push_str(&format!(
            "{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            name, stats.mp, stats.w, stats.d, stats.l, stats.p
        ));
    }
    output
}
