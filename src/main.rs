use std::error::Error;
use x11rb::protocol::Event;
use x11rb::{
    connection::Connection,
    protocol::xproto::{self, AtomEnum, ConnectionExt, EventMask, Window},
    rust_connection::RustConnection,
};

fn get_active_window(
    connection: &RustConnection,
    root: Window,
    atom: xproto::Atom,
) -> Result<Window, ()> {
    let response = connection
        .get_property::<_, u32>(false, root, atom, AtomEnum::WINDOW.into(), 0, 1)
        .unwrap()
        .reply()
        .unwrap();

    if response.value32().is_none() {
        return Err(());
    }

    Ok(response.to_owned().value32().unwrap().next().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let (connection, screen_num) = RustConnection::connect(None)?;
    let screen = &connection.setup().roots[screen_num];
    let root = screen.root;
    let net_active_window = connection
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    let active_window = get_active_window(&connection, root, net_active_window);
    if active_window.is_err() {
        eprintln!("Error getting initial active window, exiting program.");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error getting initial active window, exiting program.",
        )));
    }

    let window_id = active_window.unwrap();

    xproto::change_window_attributes(
        &connection,
        root,
        &xproto::ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
    )?;

    connection.flush()?;

    loop {
        let event = connection.wait_for_event();

        if let Ok(Event::PropertyNotify(e)) = event {
            if e.atom != net_active_window {
                continue;
            }

            let active_window = get_active_window(&connection, root, net_active_window);

            if active_window.is_err() {
                eprintln!("Error getting active window");
                continue;
            }

            if active_window.unwrap() == window_id {
                continue;
            }

            if let Err(err) = connection.unmap_window(window_id) {
                eprintln!("Error unmapping window: {:?}", err);
            } else {
                connection.flush()?;
            }
        } else {
            eprintln!("X11 server has crashed, exiting program.");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "X11 server has crashed, exiting program.",
            )));
        }
    }
}
