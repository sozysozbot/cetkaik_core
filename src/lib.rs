#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

#[cfg(test)]
mod tests {}

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
            "vessel" | "船" | "felkana" | "nuak1" => Ok(Profession::Nuak1),
            "pawn" | "兵" | "elmer" | "kauk2" => Ok(Profession::Kauk2),
            "rook" | "弓" | "gustuer" | "gua2" => Ok(Profession::Gua2),
            "bishop" | "車" | "vadyrd" | "kaun1" => Ok(Profession::Kaun1),
            "tiger" | "虎" | "stistyst" | "dau2" => Ok(Profession::Dau2),
            "horse" | "馬" | "dodor" | "maun1" => Ok(Profession::Maun1),
            "clerk" | "筆" | "kua" | "kua2" => Ok(Profession::Kua2),
            "shaman" | "巫" | "terlsk" | "tuk2" => Ok(Profession::Tuk2),
            "general" | "将" | "varxle" | "uai1" => Ok(Profession::Uai1),
            "king" | "王" | "ales" | "io" => Ok(Profession::Io),
            _ => Err(()),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" | "赤" | "kok1" => Ok(Color::Kok1),
            "black" | "黒" | "Huok2" => Ok(Color::Huok2),
            _ => Err(()),
        }
    }
}

pub mod relative;

pub mod absolute;