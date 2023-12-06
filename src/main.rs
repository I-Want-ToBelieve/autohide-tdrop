use std::error::Error;
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{self, AtomEnum, ConnectionExt, EventMask, Window},
        Event,
    },
    rust_connection::{ConnectionError, RustConnection},
};

fn get_active_window(
    connection: &RustConnection,
    root: Window,
    atom: xproto::Atom,
) -> Result<Window, Box<dyn Error>> {
    let response = connection
        .get_property::<_, u32>(false, root, atom, AtomEnum::WINDOW.into(), 0, 1)?
        .reply()?;

    response
        .value32()
        .ok_or_else(|| "No active window found".into())
        .map(|mut val| val.next().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let (connection, screen_num) = RustConnection::connect(None)?;
    let screen = &connection.setup().roots[screen_num];
    let root = screen.root;
    let net_active_window = connection
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")?
        .reply()?
        .atom;

    let window_id = get_active_window(&connection, root, net_active_window)?;

    xproto::change_window_attributes(
        &connection,
        root,
        &xproto::ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
    )?;

    connection.flush()?;

    loop {
        match connection.wait_for_event() {
            Ok(Event::PropertyNotify(e)) if e.atom == net_active_window => {
                let active_window = get_active_window(&connection, root, net_active_window)?;

                if active_window != window_id {
                    if let Err(err) = connection.unmap_window(window_id) {
                        eprintln!("Error unmapping window: {:?}", err);
                    } else {
                        connection.flush()?;
                    }
                }
            }
            Ok(_) => (),
            Err(e) => match e {
                ConnectionError::UnknownError => eprintln!("An unknown error occurred."),
                ConnectionError::UnsupportedExtension => {
                    eprintln!("An X11 extension was not supported by the server.")
                }
                ConnectionError::MaximumRequestLengthExceeded => {
                    eprintln!("A request larger than the maximum request length was sent.")
                }
                ConnectionError::FdPassingFailed => eprintln!("File descriptor passing failed."),
                ConnectionError::ParseError(err) => {
                    eprintln!("Error while parsing some data: {:?}", err)
                }
                ConnectionError::InsufficientMemory => eprintln!("Out of memory."),
                ConnectionError::IoError(_) => {
                    return Err("X11 server has crashed, exiting program.".into())
                }
                _ => eprintln!("An unexpected error occurred."),
            },
        }
    }
}
