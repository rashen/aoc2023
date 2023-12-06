pub fn split_once<'a>(input: &'a str, pat: char) -> (&'a str, &'a str) {
    let mid = input.find(pat).unwrap_or(input.len());
    let (head, tail) = input.split_at(mid);
    (&head[..mid], &tail[1..])
}
