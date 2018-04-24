// Copyright 2017-2018 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;

// use sdl2::mixer::{self, Music, INIT_OGG};

use sdl2::mixer::{self, Music, DEFAULT_CHANNELS, INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG,
                  INIT_OGG, AUDIO_S16LSB};


pub struct CoreAudio { music: Option<Music<'static>>, sounds: Vec<mixer::Chunk> }

impl CoreAudio {
    pub(crate) fn new(sound_count: u16) -> CoreAudio {
        let sounds: Vec<_> = (0..sound_count)
            .map(|id| PathBuf::from(format!("assets/sound{}.ogg", id)))
            .map(|p| mixer::Chunk::from_file(p).expect("failed to map audio"))
            .collect();
        CoreAudio { sounds, music: None }
    }

    pub fn play_sound(&mut self, sound: u16) {
        let _mixer_context = mixer::init( INIT_OGG).unwrap();
        mixer::Channel::all().play(&self.sounds[sound as usize], 0).unwrap();
    }

    pub fn loop_music(&mut self, music: u16) {
        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    
        let _mixer_context = mixer::init( INIT_OGG).expect("failed init");

        let music = &format!("assets/music{}.ogg", music);
        self.stop_music();
        let music = mixer::Music::from_file(music).expect("failed from file");
        music.play(1_000_000).expect("failed to play");
        self.music = Some(music);
    }

    pub fn stop_music(&mut self) {
        if let Some(_) = self.music.take() {
            Music::halt();
        }
    }
}
