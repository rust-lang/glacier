macro_rules! breakme {
    ($config:stmt; $($tokens:literal)*) => {
        #[cfg($config)]
        $($tokens)*
    };
}

fn main() {
    macro_rules! unix {
        () => {
            not(unix)
        };
    }

    breakme!(unix!(); "test");
}
