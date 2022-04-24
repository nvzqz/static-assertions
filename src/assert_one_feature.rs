/// Asserts that exactly one feature of a set is enabled.
/// 
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// // Both feature-1 and feature-2 are set, so this fails
/// 
/// assert_one_feature!("feature-1","foo","bar","feature-2");
/// ```
/// 
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// // None of these features are set, so this fails
/// 
/// assert_one_feature!("foo", "bar", "baz");
/// ```
/// 
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// // Exactly one of these features ("feature-1") is set, so this is fine
/// 
/// assert_one_feature!("foo", "bar", "baz", "qup", "feature-1");
/// assert_one_feature!("foo", "bar", "baz", "qup", "feature-2");
/// ```
#[macro_export]
macro_rules! assert_one_feature {
    () => {};
    ($($feature:literal),+) => {
        #[cfg(not(any($(
            feature = $feature,
        )+)))]
        compile_error!(concat!("Must have one of the following features: ", stringify!($($feature,)+)));
        const _: () = {
            $(
                #[cfg(feature = $feature)]
                const ASSERT_ONE_FEATURE:() = ();
            )+
        };
    };
}
