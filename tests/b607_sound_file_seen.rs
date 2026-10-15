//! B-607: a frame whose effect is driven by a sound notes the sound file it read, so replacing
//! that file drops the kept frame (doc 27). `audio::load` asks the disk through
//! `cache::looked_at`, the same as pictures and LUTs, both when it reads the file and when the
//! sound is already held.

use anime_compositor::{audio, cache};

#[test]
fn a_replaced_sound_file_is_seen() {
    let media = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Fixtures/audio/media");
    let file = std::env::temp_dir().join(format!("b607-sound-{}.wav", std::process::id()));
    std::fs::copy(media.join("pcm16_mono_48k.wav"), &file).unwrap();

    // Read for the first time, then again from what is held: both times the file is noted.
    for _ in 0..2 {
        let (sound, files) = cache::files_read(|| audio::load(&file));
        assert!(sound.is_ok());
        assert_eq!(
            files.iter().map(|(p, _)| p.clone()).collect::<Vec<_>>(),
            vec![file.clone()]
        );
        assert!(cache::unchanged(&files));
    }

    // Another sound in its place: the frame that read the first one is no longer trusted.
    let (_, files) = cache::files_read(|| audio::load(&file));
    std::fs::copy(media.join("pcm32_mono_48k.wav"), &file).unwrap();
    assert!(!cache::unchanged(&files));
    std::fs::remove_file(&file).unwrap();
}
