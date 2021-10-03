use std::process::Command;
macro_rules! speakers {
    () => {
        "alsa_output.pci-0000_25_00.3.analog-stereo"
    };
}
macro_rules! headphones {
    () => {
        "alsa_output.usb-Logitech_G533_Gaming_Headset-00.analog-stereo"
    };
}
pub fn get_default_sink() -> String {

    let stdout = Command::new("/usr/bin/pactl")
        .arg("get-default-sink")
        .output()
        .unwrap()
        .stdout;
        String::from_utf8(stdout).unwrap()
}
#[inline]
fn get_sink_volume(s: &str) -> String {
    let stdout = Command::new("/usr/bin/pamixer")
        .arg("--get-volume")
        .arg("--sink")
        .arg(format!("{}", s))
        .output()
        .unwrap()
        .stdout;
        String::from_utf8(stdout).unwrap()

}
pub fn get_speakers_volume() -> String {
    get_sink_volume(speakers!())
}
pub fn get_headphones_volume() -> String{
    get_sink_volume(headphones!())

}
