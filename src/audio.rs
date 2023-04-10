use bevy::prelude::{AssetServer, Res};
use bevy_kira_audio::prelude::*;


pub fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("Bob'sAdventure.mp3")).looped();
}
