pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next) = chars.peek() {
            if next == c {
                chars.next();
                count += 1;
            } else {
                break;
            }
        }
        if count > 1 {
            encoded.push_str(&count.to_string());
        }
        encoded.push(c);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count = String::new();
    for c in source.chars() {
        if c.is_digit(10) {
            count.push(c);
        } else {
            let repeat = count.parse::<usize>().unwrap_or(1);
            decoded.push_str(&c.to_string().repeat(repeat));
            count.clear();
        }
    }

    decoded
}
