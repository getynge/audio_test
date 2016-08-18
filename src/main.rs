extern crate portaudio;

use portaudio::PortAudio;

fn main() {
    let pa = PortAudio::new().unwrap();

    println!("Default host api: {:?}", pa.default_host_api());
    println!("Default host apis:");
    for host in pa.host_apis() {
        println!("{:#?}", host);
    }
    println!("HELLO WORLD1");
}