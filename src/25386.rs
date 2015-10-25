macro_rules! check_ptr_exist {
    ($var:expr, $member:ident) => (
        (*$var.c_object).$member.is_some()
    );
}
