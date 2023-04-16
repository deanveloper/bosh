//! Module for deserializing LRA tracks, aka .trk files.

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Read;

use anyhow::{anyhow, Context, Error, Result};
use bosh_rs::{Line, LineType, Vector2D};
use read_from::{LittleEndian, ReadFrom};

use crate::serialization::boshtf::{BoshTFLine, BoshTFLineType};
use crate::{BoshTFEntity, BoshTFTrack};

pub enum TrkFeature {
    RedMultiplier,
    SongInfo,
    SceneryWidth,
    IgnorableTrigger,
    ZeroStart,
    Remount,
    Frictionless,
}

impl From<&TrkFeature> for &'static str {
    fn from(feature: &TrkFeature) -> &'static str {
        match feature {
            TrkFeature::RedMultiplier => "REDMULTIPLIER",
            TrkFeature::SongInfo => "SONGINFO",
            TrkFeature::SceneryWidth => "SCENERYWIDTH",
            TrkFeature::IgnorableTrigger => "IGNORABLE_TRIGGER",
            TrkFeature::ZeroStart => "ZEROSTART",
            TrkFeature::Remount => "REMOUNT",
            TrkFeature::Frictionless => "FRICTIONLESS",
        }
    }
}

impl From<TrkFeature> for &'static str {
    fn from(feature: TrkFeature) -> Self {
        (&feature).into()
    }
}

impl Display for TrkFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl TryFrom<&str> for TrkFeature {
    type Error = Error;

    fn try_from(value: &str) -> Result<TrkFeature> {
        match value {
            "REDMULTIPLIER" => Ok(TrkFeature::RedMultiplier),
            "SONGINFO" => Ok(TrkFeature::SongInfo),
            "SCENERYWIDTH" => Ok(TrkFeature::SceneryWidth),
            "IGNORABLE_TRIGGER" => Ok(TrkFeature::IgnorableTrigger),
            "ZEROSTART" => Ok(TrkFeature::ZeroStart),
            "REMOUNT" => Ok(TrkFeature::Remount),
            "FRICTIONLESS" => Ok(TrkFeature::Frictionless),

            _ => Err(anyhow!("could not find feature for {}", value)),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum TrkLineType {
    Scenery = 0,
    Blue = 1,
    Red = 2,
}

#[derive(Default, Copy, Clone)]
pub struct TrkLineFlags(pub u8);

#[derive(Clone)]
pub struct TrkHeader {
    pub version: u8,
    pub features: HashSet<String>,

    pub song: Option<String>,
    pub start_position: Vector2D,
}

pub struct TrkMeta {
    pub entries: HashMap<String, String>,
}

pub struct TrkTrack {
    pub header: TrkHeader,
    pub lines: Vec<TrkLine>,
    pub meta: Option<TrkMeta>,
}

#[derive(Default, Clone)]
pub struct TrkLine {
    pub flags: TrkLineFlags,
    pub multiplier: u8,
    pub id: i32,
    pub line_width: u8,
    pub start: Vector2D,
    pub end: Vector2D,
}

impl TrkLineFlags {
    pub fn new(flipped: bool, extensions: (bool, bool), line_type: TrkLineType) -> TrkLineFlags {
        let mut byte = 0u8;
        byte |= (flipped as u8) << 7;
        byte |= (extensions.0 as u8) << 6;
        byte |= (extensions.1 as u8) << 5;
        byte |= line_type as u8 & 0b00011111;

        TrkLineFlags(byte)
    }
    pub fn flipped(&self) -> bool {
        self.0 & 0b10000000 > 0
    }
    pub fn extensions(&self) -> (bool, bool) {
        (self.0 & 0b00100000 > 0, self.0 & 0b01000000 > 0)
    }
    pub fn line_type(&self) -> TrkLineType {
        (self.0 & 0b00011111).into()
    }
}

// ======= DESERIALIZATION ========

impl ReadFrom for TrkLineFlags {
    type Error = Error;

    fn read_from<R: Read>(mut input: R) -> Result<TrkLineFlags> {
        let flags = u8::read_from(&mut input).context("error reading line flags")?;
        Ok(TrkLineFlags(flags))
    }
}

impl From<u8> for TrkLineType {
    fn from(value: u8) -> TrkLineType {
        match value {
            0 => TrkLineType::Scenery,
            1 => TrkLineType::Blue,
            2 => TrkLineType::Red,
            _ => TrkLineType::Scenery,
        }
    }
}

impl TrkHeader {
    fn read_from<R: Read>(mut input: R) -> anyhow::Result<TrkHeader> {
        let magic = <[u8; 4]>::read_from(&mut input)
            .context("error while reading magic value in header")?;
        if magic != [b'T', b'R', b'K', 0xF2] {
            return Err(anyhow!("magic value was not correct"));
        }

        let version = u8::read_from(&mut input).context("error while reading version in header")?;

        let features_length = LittleEndian::<u16>::read_from(&mut input)
            .context("error while reading length of features-string in header")?
            .0;

        let mut features_string = vec![0; features_length as usize];
        input
            .read_exact(features_string.as_mut_slice())
            .with_context(|| {
                format!("error while reading features-string of length {features_length} in header")
            })?;

        let features: HashSet<String> = String::from_utf8_lossy(features_string.as_slice())
            .split(';')
            .map(|s| s.to_owned())
            .collect();

        let song = if features.contains(&TrkFeature::SongInfo.to_string()) {
            let song_length =
                u8::read_from(&mut input).context("error while reading header, song length")?;
            let mut song = vec![0; song_length as usize];

            input.read_exact(song.as_mut_slice()).with_context(|| {
                format!("error while reading header, in song of length {song_length}")
            })?;

            let song = String::from_utf8_lossy(features_string.as_slice())
                .split(';')
                .map(|s| s.to_owned())
                .collect();

            Some(song)
        } else {
            None
        };

        let start_position = Vector2D::read_from(&mut input)
            .context("error while reading header, start position")?;

        Ok(TrkHeader {
            version,
            features,
            song,
            start_position,
        })
    }
}

// cannot actually implement ReadFrom trait because TrkLine requires features to be passed in
impl TrkLine {
    fn read_from<R: Read>(mut input: R, features: &HashSet<String>) -> Result<TrkLine> {
        let flags =
            TrkLineFlags(u8::read_from(&mut input).context("error while reading line flags")?);

        let multiplier = if features.contains(&TrkFeature::RedMultiplier.to_string())
            && flags.line_type() == TrkLineType::Red
        {
            u8::read_from(&mut input).context("error while reading red line multiplier")?
        } else {
            1
        };

        // red/blue line specific logic
        let id = if matches!(flags.line_type(), TrkLineType::Red | TrkLineType::Blue) {
            // ignore line triggers
            if features.contains(&TrkFeature::IgnorableTrigger.to_string()) {
                let zoom = u8::read_from(&mut input).context("error while reading zoom trigger")?;
                if zoom != 0 {
                    LittleEndian::<f32>::read_from(&mut input)
                        .context("error while reading zoom target")?;
                    LittleEndian::<i16>::read_from(&mut input)
                        .context("error while reading zoom length")?;
                }
            }

            let id = LittleEndian::<i32>::read_from(&mut input)
                .context("error while reading line id")?
                .0;

            if flags.extensions().0 || flags.extensions().1 {
                // ignore useless line extensions
                LittleEndian::<i32>::read_from(&mut input)
                    .context("error while reading line ext data 1")?;
                LittleEndian::<i32>::read_from(&mut input)
                    .context("error while reading line ext data 2")?;
            }

            id
        } else {
            0
        };

        let line_width = if features.contains(&TrkFeature::SceneryWidth.to_string())
            && matches!(flags.line_type(), TrkLineType::Scenery)
        {
            u8::read_from(&mut input).context("error while reading scenery width")?
        } else {
            1
        };

        let start = Vector2D(
            LittleEndian::<f64>::read_from(&mut input)
                .context("error while reading line start x")?
                .0,
            LittleEndian::<f64>::read_from(&mut input)
                .context("error while reading line start y")?
                .0,
        );
        let end = Vector2D(
            LittleEndian::<f64>::read_from(&mut input)
                .context("error while reading line end x")?
                .0,
            LittleEndian::<f64>::read_from(&mut input)
                .context("error while reading line end y")?
                .0,
        );

        Ok(TrkLine {
            flags,
            multiplier,
            id,
            line_width,
            start,
            end,
        })
    }
}

impl TrkMeta {
    fn read_from<R: Read>(mut input: R) -> Result<Option<TrkMeta>> {
        let magic = <[u8; 4]>::read_from(&mut input);
        if let Err(err) = &magic {
            if err.kind() == io::ErrorKind::UnexpectedEof {
                return Ok(None);
            }
        }
        let magic = magic.context("error while reading magic value in metadata")?;
        if magic != [b'M', b'E', b'T', b'A'] {
            return Err(anyhow!("magic value in metadata was not correct"));
        }

        let count = LittleEndian::<i16>::read_from(&mut input)
            .context("error while reading number of metadata entries")?
            .0;

        let mut map = HashMap::new();
        for _ in 0..count {
            let str_length = LittleEndian::<i16>::read_from(&mut input)
                .context("error while reading length of a metadata entry")?
                .0;
            let mut full_str = vec![0; str_length as usize];
            input
                .read_exact(full_str.as_mut_slice())
                .context("error while reading metadata entry")?;
            let full_str = String::from_utf8_lossy(&*full_str);

            let (key, value) = full_str
                .split_once('=')
                .with_context(|| format!("no = sign present in metadata entry: {}", full_str))?;
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Some(TrkMeta { entries: map }))
    }
}

impl ReadFrom for TrkTrack {
    type Error = Error;

    fn read_from<R: Read>(mut input: R) -> Result<TrkTrack> {
        let header =
            TrkHeader::read_from(&mut input).context("error while reading header in track")?;

        let line_count = LittleEndian::<u32>::read_from(&mut input)
            .context("error while reading line count in track")?
            .0;
        let mut lines = vec![Default::default(); line_count as usize];
        for line in &mut lines {
            *line = TrkLine::read_from(&mut input, &header.features)
                .context("error while reading line in track")?;
        }

        let meta = TrkMeta::read_from(&mut input).context("error while reading meta in track")?;

        Ok(TrkTrack {
            header,
            lines,
            meta,
        })
    }
}

// ======= TRK -> BOSHTF ========

impl TrkLineType {
    fn as_boshtf(&self, multiplier: u8) -> BoshTFLineType {
        match self {
            TrkLineType::Blue => BoshTFLineType::Normal,
            TrkLineType::Red => BoshTFLineType::Accelerate {
                amount: multiplier as u64,
            },
            TrkLineType::Scenery => BoshTFLineType::Scenery,
        }
    }
}

impl From<&TrkLine> for BoshTFLine {
    fn from(line: &TrkLine) -> Self {
        BoshTFLine::builder()
            .extension_ratio(0.25)
            .flipped(line.flags.flipped())
            .line_type(line.flags.line_type().as_boshtf(line.multiplier))
            .point_vec(line.start)
            .extended(line.flags.extensions().0)
            .point_vec(line.end)
            .extended(line.flags.extensions().1)
            .build()
    }
}

impl From<&TrkTrack> for BoshTFTrack {
    fn from(trk: &TrkTrack) -> Self {
        let zero_start = trk.header.features.contains(TrkFeature::ZeroStart.into());
        let rider = BoshTFEntity::BoshSled {
            position: trk.header.start_position,
            velocity: if zero_start {
                Vector2D(0.0, 0.0)
            } else {
                Vector2D(0.4, 0.0)
            },
        };

        BoshTFTrack {
            meta: Default::default(),
            entities: vec![rider],
            lines: trk.lines.iter().map(|l| l.into()).collect(),
        }
    }
}

// =========== BOSHTF -> TRK ===============

impl From<&LineType> for TrkLineType {
    fn from(line_type: &LineType) -> Self {
        match line_type {
            LineType::Normal => TrkLineType::Blue,
            LineType::Accelerate { .. } => TrkLineType::Red,
            LineType::Scenery => TrkLineType::Scenery,
        }
    }
}

impl From<&Line> for TrkLine {
    fn from(line: &Line) -> Self {
        TrkLine {
            flags: TrkLineFlags::new(
                line.flipped,
                (line.ends.0.extended, line.ends.1.extended),
                (&line.line_type).into(),
            ),
            multiplier: if let LineType::Accelerate { amount } = line.line_type {
                amount as u8
            } else {
                0
            },
            id: 0,
            line_width: 1,
            start: line.ends.0.location,
            end: line.ends.1.location,
        }
    }
}

impl TryFrom<&BoshTFTrack> for TrkTrack {
    type Error = Error;

    fn try_from(track: &BoshTFTrack) -> Result<TrkTrack> {
        if track.entities.len() > 1 {
            return Err(anyhow!("trk format only supports 1 entity"));
        }

        Ok(TrkTrack {
            header: TrkHeader {
                version: 1,
                features: HashSet::new(),
                song: None,
                start_position: Default::default(),
            },
            lines: vec![],
            meta: None,
        })
    }
}
