extern crate osc;
extern crate notify_rust;

use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::io::Result;

use osc::sender::*;
use osc::data::OscPacket::*;
use osc::data::OscArg::*;

use notify_rust::Notification;
use notify_rust::server::NotificationServer;

fn main()
{
    let local_addr = "localhost:7010";
    let dest_addr = "localhost:7009";

    let sender = OscSender::new(local_addr, dest_addr).unwrap();

    let mut file = File::open("/etc/hostname").unwrap();
    let mut hostname= String::new();
    file.read_to_string(&mut hostname).unwrap();


    let mut server = NotificationServer::new();
    thread::spawn(move ||
                  server.start(|notification|{
                      println!("{:#?}", notification);
                      sender.send(
                          OscMessage{
                              addr: format!("/notification/{hostname}/{app}/", app = notification.appname, hostname = hostname.trim()),
                              args: vec![
                                  OscStr(notification.summary.to_owned()),
                                  OscStr(notification.body.to_owned()),
                                  OscStr(notification.icon.to_owned()),
                                  OscInt(notification.timeout),
                              ]
                          }
                          );

                  })
                 );

    std::thread::sleep(Duration::from_millis(500));

    Notification::new()
        .summary("Notification Logger")
        .body("If you can read this in the console, the server works fine.")
        .show().unwrap();

    println!("Press enter to exit.\n");
    let mut _devnull = String::new();
    let _ = std::io::stdin().read_line(&mut _devnull);
    println!("Thank you for choosing notify-rust.");
}
