macro_rules! two_items {() => (
    extern {}
    extern {}
)}

macro_rules! single_item_funneler {( $item:item ) => ( $item )}

fn inside_some_function() {
    single_item_funneler! { two_items! {} }
}
