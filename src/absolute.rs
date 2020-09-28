use std::str::FromStr;

use super::{Color, Profession};
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Piece {
    Tam2,
    NonTam2Piece {
        color: Color,
        prof: Profession,
        side: Side,
    },
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NonTam2PieceASide {
    pub color: Color,
    pub prof: Profession,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NonTam2PieceIASide {
    pub color: Color,
    pub prof: Profession,
}

use std::collections::HashMap;
pub type Board = HashMap<Coord, Piece>;

pub struct Field {
   pub board: Board,
   pub a_side_hop1zuo1: Vec<NonTam2PieceASide>,
   pub ia_side_hop1zuo1: Vec<NonTam2PieceIASide>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

pub type Coord = (Row, Column);

#[must_use]
pub fn parse_coord(coord: &str) -> Option<(Row, Column)> {
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

    Some((row, column))
}

/// Serializes [`Coord`](../type.Coord.html).
/// # Examples
/// ```
/// use cetkaik_core::absolute::*;
///
/// assert_eq!(serialize_coord((Row::E, Column::N)), "NE");
/// assert_eq!(serialize_coord((Row::AU, Column::Z)), "ZAU");
/// ```
///
#[must_use]
pub fn serialize_coord(coord: Coord) -> String {
    let (row, column) = coord;
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
