extern crate taubyte_rs_sdk;
use taubyte_rs_sdk::event::events::Event;




#[no_mangle]
pub unsafe  fn ping(event:Event) -> u32 {
    let http = event.http().unwrap();
    _ = http.write("pong".as_bytes()).unwrap();
    
    return  0
}
