#[macro_export]
macro_rules! max {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        if a > b { a } else { b }
    }};
}

#[macro_export]
macro_rules! min {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        if a < b { a } else { b }
    }};
}