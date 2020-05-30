#[allow(dead_code)]
enum STATE {
    #[allow(non_camel_case_types)]
        S_START,
    #[allow(non_camel_case_types)]    
        S_INIT,
    #[allow(non_camel_case_types)]    
        S_WORK,
    #[allow(non_camel_case_types)]    
        S_END
} 
static M_CUR: STATE = STATE::S_START;
fn main() {
    let result = match M_CUR {
        STATE::S_START => 1,
        STATE::S_INIT  => 2,
        STATE::S_WORK  => 3,
        STATE::S_END   => 4,
    };

    println!("result={}",result);    
}
