use breadx::{display::DisplayConnection, prelude::*};

fn main() {
    /*
     * @see https://docs.rs/breadx/3.1.0/breadx/
     */
    let mut connection = DisplayConnection::connect(None).expect("should connect to x11 server");

    /*
     * @see https://gist.github.com/ssokolow/e7c9aae63fb7973e4d64cff969a78ae8
     */

    // primary event loop
    loop {
        let event = connection.wait_for_event();

        match event {
            Err(_) => {
                eprintln!("X11 server has crashed, exiting program.");
                std::process::exit(1);
            }
            Ok(_) => todo!(),
        }
    }
}
