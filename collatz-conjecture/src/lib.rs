/// Ccmputes the number of steps required to reach 1 in the collatz conjecture.
/// the Collatz conjecture is a sequence defined as follows:
/// - if n is 1, the sequence ends.
/// - if n is even, the next number is n / 2.
/// - if n is odd, the next number is 3 * n + 1.
/// the above ^^^ rules are implemented in the match expression below and is completed with:
/// - if n is zero, none is returned indicating the computation can not be completed.

pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1),
        n => {
            let next = 3u64.checked_mul(n)?.checked_add(1)?;
            collatz(next).map(|x| x + 1)
        }
    }
}
