use parsebgp_rs;
use parsebgp_rs::parsebgp_msg_t;
use parsebgp_rs::parsebgp_create_msg;

pub fn main() {
    let mut msg = unsafe { parsebgp_create_msg() };
    println!("this works");
}
