pub(crate) fn gdtw<T: PartialEq + Clone>(measure: fn(T, T) -> f64, s: Vec<T>, o: Vec<T>) -> f64 {
    assert!(!s.is_empty());
    assert!(!o.is_empty());

    fn helper<T: Clone>(
        measure: fn(T, T) -> f64,
        mut s: Vec<T>,
        mut o: Vec<T>,
        r: f64,
        l: usize,
    ) -> (f64, usize) {
        if s.len() == 1 {
            let a = s[0].clone();
            let o_len = o.len();
            (
                r + o
                    .into_iter()
                    .map(|x| measure(a.clone(), x))
                    .fold(0.0, |acc, x| acc + x),
                l + o_len,
            )
        } else if o.len() == 1 {
            helper(measure, o, s, r, l)
        } else {
            let left = measure(s[1].clone(), o[0].clone());
            let middle = measure(s[1].clone(), o[1].clone());
            let right = measure(s[0].clone(), o[1].clone());
            
            if left <= middle && left <= right {
                s.remove(0);
                helper(measure, s, o, r + left, l + 1)
            } else if middle <= left && middle <= right {
                s.remove(0);
                o.remove(0);
                helper(measure, s, o, r + middle, l + 1)
            } else if right <= left && right <= middle {
                o.remove(0);
                helper(measure, s, o, r + right, l + 1)
            } else {
                unreachable!()
            }
        }
    }

    let (a, b) = helper(
        measure,
        s.clone(),
        o.clone(),
        measure(s[0].clone(), o[0].clone()),
        1,
    );
    a / (b as f64)
}
