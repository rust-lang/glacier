#![crate_type = "lib"]

static CODEGEN: &[()] = &[];

async fn manual() {
    CODEGEN[async { 0 }.await];
}
