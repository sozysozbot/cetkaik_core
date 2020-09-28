#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

#[cfg(test)]
mod tests {}

/// Denotes the color of a piece
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    /// Red, 赤
    Kok1,

    /// Black, 黒
    Huok2,
}

/// Serializes [`Color`](./enum.Color.html).
/// # Examples
/// ```
/// use cetkaik_core::*;
///
/// assert_eq!(serialize_color(Color::Kok1), "赤");
/// assert_eq!(serialize_color(Color::Huok2), "黒");
/// ```
///
#[must_use]
pub fn serialize_color(color: Color) -> &'static str {
    match color {
        Color::Huok2 => "黒",
        Color::Kok1 => "赤",
    }
}

/// Denotes the profession of a piece
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Profession {
    /// Vessel, 船, felkana
    Nuak1,

    /// Pawn, 兵, elmer
    Kauk2,

    /// Rook, 弓, gustuer
    Gua2,

    /// Bishop, 車, vadyrd
    Kaun1,

    /// Tiger, 虎, stistyst
    Dau2,

    /// Horse, 馬, dodor
    Maun1,

    /// Clerk, 筆, kua
    Kua2,

    /// Shaman, 巫, terlsk
    Tuk2,

    /// General, 将, varxle
    Uai1,

    /// King, 王, ales
    Io,
}

/// Serializes [`Profession`](./enum.Profession.html).
/// # Examples
/// ```
/// use cetkaik_core::*;
///
/// assert_eq!(serialize_prof(Profession::Nuak1), "船");
/// assert_eq!(serialize_prof(Profession::Kaun1), "車");
/// ```
///
#[must_use]
pub fn serialize_prof(prof: Profession) -> &'static str {
    match prof {
        Profession::Nuak1 => "船",
        Profession::Kauk2 => "兵",
        Profession::Gua2 => "弓",
        Profession::Kaun1 => "車",
        Profession::Dau2 => "虎",
        Profession::Maun1 => "馬",
        Profession::Kua2 => "筆",
        Profession::Tuk2 => "巫",
        Profession::Uai1 => "将",
        Profession::Io => "王",
    }
}

use std::str::FromStr;
impl FromStr for Profession {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match &*s {
            "vessel" | "船" | "felkana" | "nuak1" | "muak1" | "pelkana" | "pijume" | "muak" => {
                Ok(Profession::Nuak1)
            }
            "pawn" | "兵" | "elmer" | "kauk2" | "elme" | "kauk" => Ok(Profession::Kauk2),
            "rook" | "弓" | "gustuer" | "gua2" | "kucte" | "kuctu" => Ok(Profession::Gua2),
            "bishop" | "車" | "车" | "vadyrd" | "kaun1" | "badut" | "xije" | "kaun" => {
                Ok(Profession::Kaun1)
            }
            "tiger" | "虎" | "stistyst" | "dau2" | "cictus" | "cucit" | "dau" => {
                Ok(Profession::Dau2)
            }
            "horse" | "馬" | "马" | "dodor" | "maun1" | "dodo" | "maun" => Ok(Profession::Maun1),
            "clerk" | "筆" | "笔" | "kua" | "kua2" | "kuwa" => Ok(Profession::Kua2),
            "shaman" | "巫" | "terlsk" | "tuk2" | "tamcuk" | "tancuk" => Ok(Profession::Tuk2),
            "general" | "将" | "varxle" | "uai1" | "baxule" | "xan" | "wai" => {
                Ok(Profession::Uai1)
            }
            "king" | "王" | "ales" | "io" | "xet" | "caupla" => Ok(Profession::Io),
            _ => Err(()),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match &*s {
            "red" | "赤" | "kok1" | "红" | "紅" => Ok(Color::Kok1),
            "black" | "黒" | "huok2" | "黑" => Ok(Color::Huok2),
            _ => Err(()),
        }
    }
}

/// Defines things in terms of relative view: "which piece is opponent's?"
pub mod relative;

/// Defines things in the absolute term: "which piece lies in the square LIA?"
pub mod absolute;

/// Defines a perspective, with which you can transform between the absolute and the relative
pub mod perspective {
    use crate::{absolute, relative};

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum Perspective {
        IaIsDown,
        IaIsUp,
    }

    impl Perspective {
        #[must_use]
        pub fn ia_is_down(self) -> bool {
            self == Perspective::IaIsDown
        }
    }

    #[must_use]
    pub fn to_absolute_side(side: relative::Side, p: Perspective) -> absolute::Side {
        match (side, p) {
            (relative::Side::Upward, Perspective::IaIsDown)
            | (relative::Side::Downward, Perspective::IaIsUp) => absolute::Side::IASide,
            (relative::Side::Downward, Perspective::IaIsDown)
            | (relative::Side::Upward, Perspective::IaIsUp) => absolute::Side::ASide,
        }
    }

    #[must_use]
    pub fn to_absolute_piece(piece: relative::Piece, p: Perspective) -> absolute::Piece {
        match piece {
            relative::Piece::Tam2 => absolute::Piece::Tam2,
            relative::Piece::NonTam2Piece { prof, color, side } => absolute::Piece::NonTam2Piece {
                prof,
                color,
                side: to_absolute_side(side, p),
            },
        }
    }

    /// Converts `relative::Coord` into `absolute::Coord`
    /// # Examples
    /// ```
    /// use cetkaik_core::*;
    /// use cetkaik_core::perspective::*;
    /// assert_eq!(
    ///     to_absolute_coord([2, 4], Perspective::IaIsDown),
    ///     (absolute::Row::I, absolute::Column::Z)
    /// )
    /// ```
    #[must_use]
    pub fn to_absolute_coord(coord: relative::Coord, p: Perspective) -> absolute::Coord {
        let [row, col] = coord;

        let columns = vec![
            absolute::Column::K,
            absolute::Column::L,
            absolute::Column::N,
            absolute::Column::T,
            absolute::Column::Z,
            absolute::Column::X,
            absolute::Column::C,
            absolute::Column::M,
            absolute::Column::P,
        ];

        let rows = vec![
            absolute::Row::A,
            absolute::Row::E,
            absolute::Row::I,
            absolute::Row::U,
            absolute::Row::O,
            absolute::Row::Y,
            absolute::Row::AI,
            absolute::Row::AU,
            absolute::Row::IA,
        ];

        (
            rows[if p.ia_is_down() { row } else { 8 - row }],
            columns[if p.ia_is_down() { col } else { 8 - col }],
        )
    }
}
