#[macro_export]
macro_rules! empty_block {
    () => { "⁅⁆" };
}

#[macro_export]
macro_rules! filled_block {
    () => { "▓▓" };
}

#[macro_export]
macro_rules! n_write {
    ($out:expr, $c:expr, $count:expr) => {
        for _ in 0..$count {
            write!($out, "{}", $c)?;
        }
    };
}