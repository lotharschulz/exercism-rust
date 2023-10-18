pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => format!("{0} bottle of beer on the wall, {1} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n),
        _ => format!("{0} bottles of beer on the wall, {1} bottles of beer.\nTake one down and pass it around, {2} {3} of beer on the wall.\n", n, n, n-1, if n == 2 { "bottle" } else { "bottles" })
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result: String = String::from("");
    for i in (end..=start).rev() {
        let str = verse(i);
        if i == start { result = format!("{0}", str); } 
        else          { result = format!("{0}\n{1}", result, str); }
    }
    result
}
