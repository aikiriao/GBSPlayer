// test時はno_stdを無効に設定
#![cfg_attr(not(test), no_std)]
pub mod types;
pub mod gbs_file;
pub mod gbs_player;
mod assembler;
mod sm83;
mod sound_generator;
mod apu;
mod midiapu;
