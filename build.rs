use std::env;

fn main(){
    env::set_var("PORTAUDIO_ONLY_STATIC", "TRUE");
    assert_eq!(env::var("PORTAUDIO_ONLY_STATIC"), Ok("TRUE".to_string()));
}