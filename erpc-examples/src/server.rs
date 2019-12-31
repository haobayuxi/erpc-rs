use erpc_sys::ffi;
use std::os::raw::{c_int, c_void};
use libc::{size_t};
use erpc_rs::context::AppContext;
use erpc_rs::rpc::Rpc;
use erpc_rs::nexus::Nexus;

extern fn req_handler(req_handle: *mut ffi::ReqHandle, context: *mut c_void) -> () {
    println!("req_handler start");
    let data: *mut u8;
    let data_size : size_t = 0;
    unsafe { data = ffi::erpc_get_req_msgbuf(req_handle, &data_size) };
    //println!("data_size: {:?} {}", data, data_size);

    let s = unsafe { String::from_raw_parts(data, data_size, 0) };
    println!("req: {}", s);

    let ctx: *mut ffi::AppContext = context as *mut ffi::AppContext;
    let _rpc = unsafe { ffi::app_context_rpc(ctx) };
    let s = "world".to_string();
    unsafe { ffi::erpc_enqueue_response(_rpc, req_handle, s.as_ptr(), s.len()) };
    println!("req_handler end");
}

extern fn sm_handler(_session_num: c_int, _sm_event_type: ffi::SmEventType, _sm_err_type: ffi::SmErrType, _context: *mut c_void) {
    println!("sm_handler");
}

fn main() {
    // sudo rxe_cfg start
    // sudo rxe_cfg status
    let context = AppContext::new();
    let mut nexus = Nexus::new("127.0.0.1:31850".to_string(), 0, 0);

    nexus.register_req_func(1, req_handler, 0);

    let mut rpc = Rpc::new(&context, &nexus, 0, sm_handler, 0);

    loop {
        rpc.run_event_loop(1000);
    }
}
