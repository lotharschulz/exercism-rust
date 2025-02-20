pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    for i in 0..take_down {
        let current_bottles = start_bottles - i;
        if i > 0 {
            song.push_str("\n\n");
        }

        song.push_str(&match current_bottles {
            2 => format!(
                "Two green bottles hanging on the wall,\n\
                 Two green bottles hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be one green bottle hanging on the wall."
            ),
            1 => format!(
                "One green bottle hanging on the wall,\n\
                 One green bottle hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be no green bottles hanging on the wall."
            ),
            0 => format!(
                "No green bottles hanging on the wall,\n\
                 No green bottles hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be no green bottles hanging on the wall."
            ),
            n => {
                let next_bottles = current_bottles - 1;
                let next_bottles_str = if next_bottles == 1 {
                    "one".to_string()
                } else {
                    match next_bottles {
                        2 => "two".to_string(),
                        3 => "three".to_string(),
                        4 => "four".to_string(),
                        5 => "five".to_string(),
                        6 => "six".to_string(),
                        7 => "seven".to_string(),
                        8 => "eight".to_string(),
                        9 => "nine".to_string(),
                        10 => "ten".to_string(),
                        _ => next_bottles.to_string(),
                    }
                };
                format!(
                    "{} green bottles hanging on the wall,\n\
                     {} green bottles hanging on the wall,\n\
                     And if one green bottle should accidentally fall,\n\
                     There'll be {} green bottle{} hanging on the wall.",
                    match n {
                        2 => "Two".to_string(),
                        3 => "Three".to_string(),
                        4 => "Four".to_string(),
                        5 => "Five".to_string(),
                        6 => "Six".to_string(),
                        7 => "Seven".to_string(),
                        8 => "Eight".to_string(),
                        9 => "Nine".to_string(),
                        10 => "Ten".to_string(),
                        _ => n.to_string(),
                    },
                    match n {
                        2 => "Two".to_string(),
                        3 => "Three".to_string(),
                        4 => "Four".to_string(),
                        5 => "Five".to_string(),
                        6 => "Six".to_string(),
                        7 => "Seven".to_string(),
                        8 => "Eight".to_string(),
                        9 => "Nine".to_string(),
                        10 => "Ten".to_string(),
                        _ => n.to_string(),
                    },
                    next_bottles_str,
                    if next_bottles == 1 { "" } else { "s" }
                )
            }
        });
    }
    song
}
