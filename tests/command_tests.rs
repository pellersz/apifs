use {
    apifs::{
        option::parse_option      
    }
};

#[test]
pub fn option_test() {
    parse_option("fuc");
}

