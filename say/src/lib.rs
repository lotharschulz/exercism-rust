pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut result = String::new();
    let mut num = n;

    let scales = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    let mut i = 0;
    while num > 0 {
        let chunk = num % 1000;
        num /= 1000;

        if chunk != 0 {
            let chunk_str = encode_chunk(chunk);
            if !result.is_empty() {
                result = format!("{} {} {}", chunk_str, scales[i], result);
            } else {
                result = format!("{} {}", chunk_str, scales[i],);
            }
            result = result.trim().to_string();
        }
        i += 1;
    }

    result
}

fn encode_chunk(n: u64) -> String {
    let mut result = String::new();

    let hundreds = n / 100;
    let remainder = n % 100;

    if hundreds > 0 {
        result.push_str(&format!("{} hundred", encode_unit(hundreds)));
        if remainder > 0 {
            result.push_str(" ");
        }
    }

    if remainder > 0 {
        result.push_str(&encode_tens(remainder));
    }

    result
}

fn encode_tens(n: u64) -> String {
    if n < 20 {
        encode_unit(n).to_string()
    } else {
        let tens = n / 10;
        let units = n % 10;

        let mut result = match tens {
            2 => "twenty".to_string(),
            3 => "thirty".to_string(),
            4 => "forty".to_string(),
            5 => "fifty".to_string(),
            6 => "sixty".to_string(),
            7 => "seventy".to_string(),
            8 => "eighty".to_string(),
            9 => "ninety".to_string(),
            _ => "".to_string(),
        };

        if units > 0 {
            result.push_str(&format!("-{}", encode_unit(units)));
        }

        result
    }
}

fn encode_unit(n: u64) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }
}
