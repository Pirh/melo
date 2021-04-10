use crate::error::SourceLoc;
use crate::notes::Midi;

#[derive(Debug, PartialEq)]
pub struct Piece<'a> {
    pub title: Option<&'a str>,
    pub composer: Option<&'a str>,
    pub tempo: u64,
    pub beats: u64,

    pub voices: Vec<Voice<'a>>,
}

impl<'a> Default for Piece<'a> {
    fn default() -> Self {
        Piece {
            title: None,
            composer: None,
            tempo: 120,
            beats: 4,
            voices: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Voice<'a> {
    pub name: &'a str,
    pub channel: u8,
    pub program: u8,
    pub transpose: i8,
    pub volume: Option<f64>,
    pub notes: Vec<Note>,
    pub divisions_per_bar: u32,
    pub debug_bar_info: Vec<DebugBarInfo>,
}

impl<'a> Default for Voice<'a> {
    fn default() -> Self {
        Voice {
            name: "error",
            channel: 1,
            program: 0,
            transpose: 0,
            volume: None,
            notes: Vec::new(),
            divisions_per_bar: 1,
            debug_bar_info: Vec::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Note {
    pub position: u32,
    pub length: u32,
    pub midi: Midi,
}

#[derive(Debug, PartialEq)]
pub struct DebugBarInfo {
    pub loc: SourceLoc,
    pub divisions_in_source: u32,
}
