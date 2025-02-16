use core::panic;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

enum Event {
    ActiveWindow {
        window_class: String,
        window_title: String,
    },

    Unrecognized {
        name: String,
        fields: Vec<String>,
    },
}

fn parse(event: &str) -> Event {
    if let Some((kind, payload)) = event.split_once(">>") {
        let fields: Vec<&str> = payload.split(",").collect();
        return match kind {
            "activewindow" => Event::ActiveWindow {
                window_class: fields[0].to_string(),
                window_title: fields[1].to_string(),
            },
            _ => Event::Unrecognized {
                name: kind.to_string(),
                fields: fields.iter().map(|x| x.to_string()).collect(),
            },
        };
    }
    panic!("Could not parse Hyprland event");
}

fn event_socket_path() -> PathBuf {
    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap();
    return Path::new(&xdg_runtime_dir)
        .join("hypr")
        .join(his)
        .join(".socket2.sock");
}

fn accept(event: Event) {
    match event {
        Event::ActiveWindow {
            window_class,
            window_title,
        } => println!("{window_title} {window_class}"),
        _ => (),
    }
}

fn main() {
    let path = event_socket_path();
    let x = path.display();
    println!("Connecting to {x}");

    let socket = match UnixStream::connect(path) {
        Ok(sock) => sock,
        Err(e) => {
            println!("could not connect {e}");
            return;
        }
    };

    let mut raw_event = String::new();
    let mut reader = BufReader::new(socket);

    while let Ok(count) = reader.read_line(&mut raw_event) {
        if count > 0 {
            let event = parse(&raw_event);
            accept(event);
        }
    }
}
