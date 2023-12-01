#[macro_export]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        crate::util::compose_two($head, crate::util::compose!($($tail),+))
    };
}

pub(crate) use compose;

pub fn compose_two<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a| g(f(a))
}

pub fn non_empty_lines(s: &str) -> impl Iterator<Item = String> + '_ {
    s.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}
