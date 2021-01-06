use super::{Color, Profession};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Piece {
    Tam2,
    NonTam2Piece {
        color: Color,
        prof: Profession,
        side: Side,
    },
}

#[must_use]
pub fn distance(a: Coord, b: Coord) -> i32 {
    use super::{perspective, relative};
    // coordinate-independent, so I can just choose one
    relative::distance(
        perspective::to_relative_coord(a, perspective::Perspective::IaIsDownAndPointsUpward),
        perspective::to_relative_coord(b, perspective::Perspective::IaIsDownAndPointsUpward),
    )
}

impl Piece {
    #[must_use]
    pub const fn is_tam2(self) -> bool {
        match self {
            Piece::Tam2 => true,
            Piece::NonTam2Piece { .. } => false,
        }
    }
    #[must_use]
    pub fn has_color(self, clr: Color) -> bool {
        match self {
            Piece::Tam2 => false,
            Piece::NonTam2Piece { color, .. } => color == clr,
        }
    }
    #[must_use]
    pub fn has_prof(self, prf: Profession) -> bool {
        match self {
            Piece::Tam2 => false,
            Piece::NonTam2Piece { prof, .. } => prof == prf,
        }
    }
    #[must_use]
    pub fn has_side(self, sid: Side) -> bool {
        match self {
            Piece::Tam2 => false,
            Piece::NonTam2Piece { side, .. } => side == sid,
        }
    }
}

#[must_use]
pub const fn is_water(Coord(row, col): Coord) -> bool {
    match row {
        Row::O => matches!(
            col,
            Column::N | Column::T | Column::Z | Column::X | Column::C
        ),
        Row::I | Row::U | Row::Y | Row::AI => matches!(col, Column::Z),
        _ => false,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct NonTam2Piece {
    pub color: Color,
    pub prof: Profession,
}

impl std::fmt::Display for NonTam2Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            super::serialize_color(self.color),
            super::serialize_prof(self.prof)
        )
    }
}
use std::convert::TryInto;
impl TryInto<NonTam2Piece> for &str {
    type Error = ();
    fn try_into(self) -> Result<NonTam2Piece, Self::Error> {
        Ok(match self {
            "黒兵" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
            },
            "赤兵" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
            },
            "黒弓" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Gua2,
            },
            "黒車" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kaun1,
            },
            "黒虎" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Dau2,
            },
            "黒馬" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Maun1,
            },
            "黒筆" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kua2,
            },
            "黒巫" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Tuk2,
            },
            "黒将" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Uai1,
            },
            "赤弓" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Gua2,
            },
            "赤車" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kaun1,
            },
            "赤虎" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Dau2,
            },
            "赤馬" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Maun1,
            },
            "赤筆" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kua2,
            },
            "赤巫" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Tuk2,
            },
            "赤将" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Uai1,
            },
            "黒王" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Io,
            },
            "赤王" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Io,
            },
            "黒船" => NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Nuak1,
            },
            "赤船" => NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Nuak1,
            },
            _ => return Err(()),
        })
    }
}

use std::collections::HashMap;
pub type Board = HashMap<Coord, Piece>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub board: Board,
    pub a_side_hop1zuo1: Vec<NonTam2Piece>,
    pub ia_side_hop1zuo1: Vec<NonTam2Piece>,
}

impl Field {
    pub fn insert_nontam_piece_into_hop1zuo1(
        &mut self,
        color: Color,
        prof: Profession,
        side: Side,
    ) {
        match side {
            Side::ASide => self.a_side_hop1zuo1.push(NonTam2Piece { color, prof }),
            Side::IASide => self.ia_side_hop1zuo1.push(NonTam2Piece { color, prof }),
        }
    }
    #[must_use]
    pub fn find_and_remove_piece_from_hop1zuo1(
        &self,
        color: Color,
        prof: Profession,
        side: Side,
    ) -> Option<Self> {
        match side {
            Side::ASide => {
                let mut that = self.clone();
                let index = that
                    .a_side_hop1zuo1
                    .iter()
                    .position(|x| *x == NonTam2Piece { color, prof })?;
                that.a_side_hop1zuo1.remove(index);
                Some(that)
            }
            Side::IASide => {
                let mut that = self.clone();
                let index = that
                    .ia_side_hop1zuo1
                    .iter()
                    .position(|x| *x == NonTam2Piece { color, prof })?;
                that.ia_side_hop1zuo1.remove(index);
                Some(that)
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash, Deserialize, Serialize)]
pub enum Side {
    ASide,
    IASide,
}

impl FromStr for Side {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Side::ASide),
            "IA" => Ok(Side::IASide),
            _ => Err(()),
        }
    }
}

use std::ops;
impl ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::ASide => Side::IASide,
            Side::IASide => Side::ASide,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Row {
    A,
    E,
    I,
    U,
    O,
    Y,
    AI,
    AU,
    IA,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Column {
    K,
    L,
    N,
    T,
    Z,
    X,
    C,
    M,
    P,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub struct Coord(pub Row, pub Column);

impl serde::ser::Serialize for Coord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&serialize_coord(*self))
    }
}

struct CoordVisitor;

impl<'de> serde::de::Visitor<'de> for CoordVisitor {
    type Value = Coord;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a coordinate")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Coord::from_str(s) {
            Ok(c) => Ok(c),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &self,
            )),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Coord {
    fn deserialize<D>(deserializer: D) -> Result<Coord, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(CoordVisitor)
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_coord(s).ok_or(())
    }
}

/// Parses [`Coord`](type.Coord.html).
/// # Examples
/// ```
/// use cetkaik_core::absolute::*;
/// assert_eq!(
///     parse_coord("LIA"),
///     Some(Coord(Row::IA, Column::L))
/// );
///
/// // case-sensitive
/// assert_eq!(
///     parse_coord("LiA"),
///     None
/// );
/// ```
#[must_use]
pub fn parse_coord(coord: &str) -> Option<Coord> {
    if coord.is_empty() || coord.len() > 3 {
        return None;
    }

    let column = match coord.chars().next() {
        Some('C') => Some(Column::C),
        Some('K') => Some(Column::K),
        Some('L') => Some(Column::L),
        Some('M') => Some(Column::M),
        Some('N') => Some(Column::N),
        Some('P') => Some(Column::P),
        Some('T') => Some(Column::T),
        Some('X') => Some(Column::X),
        Some('Z') => Some(Column::Z),
        None | Some(_) => None,
    }?;

    let row = match &coord[1..coord.len()] {
        "A" => Some(Row::A),
        "AI" => Some(Row::AI),
        "AU" => Some(Row::AU),
        "E" => Some(Row::E),
        "I" => Some(Row::I),
        "O" => Some(Row::O),
        "U" => Some(Row::U),
        "Y" => Some(Row::Y),
        "IA" => Some(Row::IA),
        _ => None,
    }?;

    Some(Coord(row, column))
}

#[must_use]
pub fn yhuap_initial_board() -> Board {
    hashmap! {
        Coord(Row::O, Column::Z) => Piece::Tam2,
        Coord(Row::AI, Column::Z) => Piece::NonTam2Piece {prof: Profession::Nuak1, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AI, Column::K) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AI, Column::N) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AI, Column::C) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AI, Column::P) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AI, Column::L) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::AI, Column::T) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::AI, Column::X) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::AI, Column::M) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::AU, Column::L) => Piece::NonTam2Piece {prof: Profession::Gua2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AU, Column::M) => Piece::NonTam2Piece {prof: Profession::Gua2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::IA, Column::C) => Piece::NonTam2Piece {prof: Profession::Kaun1, color: Color::Huok2, side: Side::IASide},
        Coord(Row::IA, Column::N) => Piece::NonTam2Piece {prof: Profession::Kaun1, color: Color::Kok1, side: Side::IASide},
        Coord(Row::AU, Column::T) => Piece::NonTam2Piece {prof: Profession::Dau2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AU, Column::X) => Piece::NonTam2Piece {prof: Profession::Dau2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::IA, Column::M) => Piece::NonTam2Piece {prof: Profession::Maun1, color: Color::Huok2, side: Side::IASide},
        Coord(Row::IA, Column::L) => Piece::NonTam2Piece {prof: Profession::Maun1, color: Color::Kok1, side: Side::IASide},
        Coord(Row::IA, Column::P) => Piece::NonTam2Piece {prof: Profession::Kua2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::AU, Column::P) => Piece::NonTam2Piece {prof: Profession::Tuk2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::IA, Column::X) => Piece::NonTam2Piece {prof: Profession::Uai1, color: Color::Huok2, side: Side::IASide},
        Coord(Row::IA, Column::T) => Piece::NonTam2Piece {prof: Profession::Uai1, color: Color::Kok1, side: Side::IASide},
        Coord(Row::IA, Column::Z) => Piece::NonTam2Piece {prof: Profession::Io, color: Color::Huok2, side: Side::IASide},
        Coord(Row::IA, Column::K) => Piece::NonTam2Piece {prof: Profession::Kua2, color: Color::Kok1, side: Side::IASide},
        Coord(Row::AU, Column::K) => Piece::NonTam2Piece {prof: Profession::Tuk2, color: Color::Huok2, side: Side::IASide},
        Coord(Row::I, Column::Z) => Piece::NonTam2Piece {prof: Profession::Nuak1, color: Color::Kok1, side: Side::ASide},
        Coord(Row::I, Column::K) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::I, Column::N) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::I, Column::C) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::I, Column::P) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::I, Column::L) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::I, Column::T) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::I, Column::X) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::I, Column::M) => Piece::NonTam2Piece {prof: Profession::Kauk2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::E, Column::M) => Piece::NonTam2Piece {prof: Profession::Gua2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::E, Column::L) => Piece::NonTam2Piece {prof: Profession::Gua2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::A, Column::N) => Piece::NonTam2Piece {prof: Profession::Kaun1, color: Color::Huok2, side: Side::ASide},
        Coord(Row::A, Column::C) => Piece::NonTam2Piece {prof: Profession::Kaun1, color: Color::Kok1, side: Side::ASide},
        Coord(Row::E, Column::X) => Piece::NonTam2Piece {prof: Profession::Dau2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::E, Column::T) => Piece::NonTam2Piece {prof: Profession::Dau2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::A, Column::L) => Piece::NonTam2Piece {prof: Profession::Maun1, color: Color::Huok2, side: Side::ASide},
        Coord(Row::A, Column::M) => Piece::NonTam2Piece {prof: Profession::Maun1, color: Color::Kok1, side: Side::ASide},
        Coord(Row::A, Column::K) => Piece::NonTam2Piece {prof: Profession::Kua2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::E, Column::P) => Piece::NonTam2Piece {prof: Profession::Tuk2, color: Color::Huok2, side: Side::ASide},
        Coord(Row::A, Column::P) => Piece::NonTam2Piece {prof: Profession::Kua2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::E, Column::K) => Piece::NonTam2Piece {prof: Profession::Tuk2, color: Color::Kok1, side: Side::ASide},
        Coord(Row::A, Column::T) => Piece::NonTam2Piece {prof: Profession::Uai1, color: Color::Huok2, side: Side::ASide},
        Coord(Row::A, Column::X) => Piece::NonTam2Piece {prof: Profession::Uai1, color: Color::Kok1, side: Side::ASide},
        Coord(Row::A, Column::Z) => Piece::NonTam2Piece {prof: Profession::Io, color: Color::Huok2, side: Side::ASide},
    }
}

/// Serializes [`Coord`](../type.Coord.html).
/// # Examples
/// ```
/// use cetkaik_core::absolute::*;
///
/// assert_eq!(serialize_coord(Coord(Row::E, Column::N)), "NE");
/// assert_eq!(serialize_coord(Coord(Row::AU, Column::Z)), "ZAU");
/// ```
///
#[must_use]
pub fn serialize_coord(coord: Coord) -> String {
    let Coord(row, column) = coord;
    format!(
        "{}{}",
        match column {
            Column::K => "K",
            Column::L => "L",
            Column::M => "M",
            Column::N => "N",
            Column::P => "P",
            Column::Z => "Z",
            Column::X => "X",
            Column::C => "C",
            Column::T => "T",
        },
        match row {
            Row::A => "A",
            Row::E => "E",
            Row::I => "I",
            Row::O => "O",
            Row::U => "U",
            Row::Y => "Y",
            Row::IA => "IA",
            Row::AI => "AI",
            Row::AU => "AU",
        }
    )
}
