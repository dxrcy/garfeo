/// until added to ibex crate
pub fn is_local() -> bool {
    std::env::args().nth(1) == Some("local".to_string())
}

/// until added to ibex crate
macro_rules! url {
    () => {{
        url!(@root)
    }};
    ($path:expr) => {{
        format!("{}{}", url!(@root), $path.to_string())
    }};
    (@root) => {
        if features::is_local() { "/" } else { URL_ROOT }
    };
}
