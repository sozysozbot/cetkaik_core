#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::non_ascii_literal, clippy::use_self)]
#[macro_use]
extern crate maplit;
/// Denotes the color of a piece
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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
pub const fn serialize_color(color: Color) -> &'static str {
    match color {
        Color::Huok2 => "黒",
        Color::Kok1 => "赤",
    }
}

/// Denotes the profession of a piece
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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
pub const fn serialize_prof(prof: Profession) -> &'static str {
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

    /// Parses [`Profession`](./enum.Profession.html).
    /// # Examples
    /// ```
    /// use cetkaik_core::*;
    ///
    /// assert_eq!("船".parse(), Ok(Profession::Nuak1));
    /// assert_eq!("elmer".parse(), Ok(Profession::Kauk2));
    /// assert_eq!("车".parse(), Ok(Profession::Kaun1));
    /// assert_eq!("uai1".parse(), Ok(Profession::Uai1));
    /// ```
    ///
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
pub mod perspective;

impl serde::ser::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(serialize_color(*self))
    }
}

impl serde::ser::Serialize for Profession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(serialize_prof(*self))
    }
}

struct ColorVisitor;

impl<'de> serde::de::Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a color")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Color::from_str(s) {
            Ok(c) => Ok(c),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &self,
            )),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

struct ProfessionVisitor;

impl<'de> serde::de::Visitor<'de> for ProfessionVisitor {
    type Value = Profession;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a profession")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Profession::from_str(s) {
            Ok(c) => Ok(c),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &self,
            )),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Profession {
    fn deserialize<D>(deserializer: D) -> Result<Profession, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(ProfessionVisitor)
    }
}
