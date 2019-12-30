#[doc(hidden)]
#[macro_export]
macro_rules! _head {
    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head
    };
}
