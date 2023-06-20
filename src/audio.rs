use std::io::BufReader;
use rodio::OutputStreamHandle;
use crate::chess::MoveKind;

pub fn play_sound(move_kind: MoveKind, handle: &OutputStreamHandle) {
    let sound_file_path = match move_kind {
        MoveKind::MoveSelf => "assets/move-self.wav",
        MoveKind::Capture => "assets/capture.wav",

        _ => todo!("not implemented"),
    };

    play_wav_from_path(sound_file_path, handle).expect("Failed to load sound: {sound_file_path}");
}

fn play_wav_from_path(file_path: &str, handle: &OutputStreamHandle) -> Result<(), std::io::Error>{
    let wav_file = std::fs::File::open(file_path)?;

    let sound = handle.play_once(BufReader::new(wav_file)).unwrap();
    sound.set_volume(0.3);
    sound.detach();
    Ok(())
}
