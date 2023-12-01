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
