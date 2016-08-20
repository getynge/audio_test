use std::env;

/*
BUILD INSTRUCTIONS FOR WINDOWS:
This will  build more or less automatically on everything else, but on any version of windows
other than 7 the process is somewhat involved.
For now (considering this is a test program I'm probably never changing this) do the following:
1. Download PortAudio source code
2. Use CMAKE to configure PortAudio
3. Be sure to disable ASIO and DirectX Sound and have Static Compilation enabled
4. Generate Build files with CMAKE
5. Compile project file using visual studio
6. Create a folder in the root directory of audio_test called build_requirements
7. rename the compile library to portaudio.lib and copy it to build_requirements
8. Download Windows 10/8.1/8/7 SDK
9. Copy ksguid.lib to build_requirements
10. execute Cargo run
*/
fn main(){
    //TODO: Change this so users can configure it
    println!("cargo:rustc-link-search=native=build_requirements")
}