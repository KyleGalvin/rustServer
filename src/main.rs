use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::io::ErrorKind;
use std::thread;
use std::str;
use std::string::String;

pub struct ControllerState {
    player_id: i32,
    button_a: bool,
    button_b: bool,
    button_x: bool,
    button_y: bool,
    button_start: bool,
    button_back: bool,
    button_left: bool,
    button_right: bool,
    button_up: bool,
    button_down: bool,
    button_r: bool,
    button_l: bool,
    left_x:  f32,
    left_y: f32,
    right_x: f32,
    right_y: f32,
    trigger_l: f32,
    trigger_r: f32
}

impl std::fmt::Display for ControllerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}\t{}{}{}{}{}{}{}{}{}{}{}{} {} {} {} {} {} {})", 
            self.player_id as i32,
            self.button_x as i32, 
            self.button_a as i32, 
            self.button_b as i32, 
            self.button_y as i32,
            self.button_up as i32, 
            self.button_down as i32, 
            self.button_left as i32, 
            self.button_right as i32,
            self.button_start as i32, 
            self.button_back as i32, 
            self.button_l as i32, 
            self.button_r as i32,
            self.left_x, 
            self.left_y, 
            self.right_x, 
            self.right_y, 
            self.trigger_l, 
            self.trigger_r)
    }
}

pub trait StringExt {
    fn to_controller_state(&self) -> ControllerState;
}

fn GetNthCharacter(string: &str, n: usize) -> char {
    return string.chars().nth(n).unwrap_or_default();
}

fn UnpackBitFromString(string: &str, n: usize) -> bool {
    return if GetNthCharacter(string, n) == '1' { true } else { false };
}

impl StringExt for str {
    fn to_controller_state(&self) -> ControllerState {
        let mut iter = self.split_whitespace();

        let binary_buttons = iter.next().unwrap_or("000000000000");
        let left_x = iter.next().unwrap_or("0.0");
        let left_y = iter.next().unwrap_or("0.0");
        let right_x = iter.next().unwrap_or("0.0");
        let right_y = iter.next().unwrap_or("0.0");
        let trigger_l = iter.next().unwrap_or("0.0");
        let trigger_r = iter.next().unwrap_or("0.0");
        let player_id = iter.next().unwrap_or("0").trim();
        println!("playerid {} len {}", player_id, player_id.len());

        let s = ControllerState {
            player_id : player_id.parse().unwrap_or(0),
            button_left : UnpackBitFromString(binary_buttons, 0),
            button_down : UnpackBitFromString(binary_buttons, 1), 
            button_up : UnpackBitFromString(binary_buttons, 2),
            button_right : UnpackBitFromString(binary_buttons, 3),
            button_x : UnpackBitFromString(binary_buttons, 4),
            button_a : UnpackBitFromString(binary_buttons, 5),
            button_b : UnpackBitFromString(binary_buttons, 6),
            button_y : UnpackBitFromString(binary_buttons, 7),
            button_r : UnpackBitFromString(binary_buttons, 8),
            button_l : UnpackBitFromString(binary_buttons, 9),
            button_start : UnpackBitFromString(binary_buttons, 10),
            button_back : UnpackBitFromString(binary_buttons, 11),
            left_x : left_x.parse().unwrap_or(0.0),
            left_y : left_y.parse().unwrap_or(0.0), 
            right_x : right_x.parse().unwrap_or(0.0),
            right_y : right_y.parse().unwrap_or(0.0),
            trigger_l : trigger_l.parse().unwrap_or(0.0),
            trigger_r : trigger_r.parse().unwrap_or(0.0)
        };
        return s;
    }
}

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:12345").expect("Failed to bind to socket");
    println!("Hello, world!");

    let mut buf = [0u8; 1024];
    let mut exit = false;
    while !exit {
        let result = sock.recv(&mut buf);
        match result {
            // If `recv` was successfull, print the number of bytes received.
            // The received data is stored in `buf`.
            Ok(num_bytes) => {
                let byte_string = str::from_utf8(buf.get(0..num_bytes).unwrap()).unwrap();
                println!("I received {} bytes! {}. len {}", num_bytes, byte_string, byte_string.len());
                if num_bytes > 16 {
                    println!("Controller state {}", byte_string.to_controller_state());
                }
                
                
            },
            // If we get an error other than "would block", print the error.
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                println!("Something went wrong: {}", err)
            }
            // Do nothing otherwise.
            _ => {}
        }

        thread::sleep(Duration::from_millis(5));
    }

    println!("Done");
}
