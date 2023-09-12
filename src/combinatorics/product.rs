/// Take a Vec of Vecs
/// Returns a vec of tuples
/// <https://stackoverflow.com/a/34966711/9518712>
pub fn product() {
    const N: usize = 3;
    let mut result = Vec::new();
    let pools = [
        (0..1).collect::<Vec<_>>(),
        (0..1).collect::<Vec<_>>(),
        (0..1).collect::<Vec<_>>(),
    ];
    let mut indices = [0; N];

    let mut finished = false;
    while !finished {
        // build current state
        let mut next: [usize; N] = Default::default();
        for (i, value) in indices
            .iter()
            .zip(pools.iter())
            .map(|(i, p)| p[*i])
            .enumerate()
        {
            next[i] = value;
        }
        result.push(next);

        // "carry" logic
        for i in (0..pools.len()).rev() {
            indices[i] += 1;
            if indices[i] >= pools[i].len() {
                indices[i] = 0;
            } else {
                break;
            }
            finished = i == 0;
        }
    }
    println!("{result:?}");
}
