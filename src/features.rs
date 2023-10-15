/// until added to ibex crate
macro_rules! url {
    ( $path:expr ) => {{
        let root = if std::env::args().nth(1) == Some("local".to_string()) {
            ""
        } else {
            URL_ROOT
        };
        format!("{}{}", root, $path.to_string())
    }};
}
/// until `for` works in macro
macro_rules! foreach {
    ( $pat:pat in $expr:expr => $($tt:tt)* ) => {
        ($expr).map(|$pat| view! { $($tt)* }).collect::<Vec<View>>()
    }
}
