#![feature(cfg_eval)]
#![feature(stmt_expr_attributes)]

#[cfg_eval]
fn main() {
    #[cfg_eval]
    let _ = #[cfg(FALSE)] 0;
    //~^ ERROR removing an expression is not supported in this position
}
