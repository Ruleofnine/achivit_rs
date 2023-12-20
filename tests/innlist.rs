use achivit_rs::roles::{get_InnList};

#[test]
fn inn_loads(){
    assert_eq!(true,get_InnList().is_ok())
}
