/// Macro to compose a sequence of links into a chain (ergonomic API).
/// Usage: let my_chain = chain![link1, link2, link3];
#[macro_export]
macro_rules! chain {
    ( $($link:expr),* $(,)? ) => {{
        let mut chain = $crate::chains::Chain::new();
        $( chain.add_link($link); )*
        chain
    }};
}
