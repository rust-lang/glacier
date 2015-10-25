macro_rules! test_cases {
    (
        $(#[$case_name:ident($($test_case:expr),+)])+
        $test_name:ident ($($param_name:ident),+)
        $(use $paths::path;)*
        $test_code:block
    ) => ();
}

test_cases!(
    #[a(1)]
    t (c) { }
);
