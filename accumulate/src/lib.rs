/// Transform each element of the input collection using the provided function
pub fn map<T, U, F, I>(input: I, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
    I: IntoIterator<Item = T>,
{
    let input = input.into_iter();
    let mut result = Vec::with_capacity(input.size_hint().0);

    for item in input {
        result.push(function(item));
    }

    result
}
