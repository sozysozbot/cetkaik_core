use super::{Color, Profession};
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Side {
    Upward,
    Downward,
}

impl std::ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Upward => Side::Downward,
            Side::Downward => Side::Upward,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NonTam2PieceDownward {
    pub color: Color,
    pub prof: Profession,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NonTam2PieceUpward {
    pub color: Color,
    pub prof: Profession,
}

impl From<NonTam2PieceUpward> for Piece {
    fn from(from: NonTam2PieceUpward) -> Piece {
        Piece::NonTam2Piece {
            color: from.color,
            prof: from.prof,
            side: Side::Upward,
        }
    }
}

impl From<NonTam2PieceDownward> for Piece {
    fn from(from: NonTam2PieceDownward) -> Piece {
        Piece::NonTam2Piece {
            color: from.color,
            prof: from.prof,
            side: Side::Downward,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Piece {
    Tam2,
    NonTam2Piece {
        color: Color,
        prof: Profession,
        side: Side,
    },
}

#[must_use]
pub fn rotate_piece_or_null(p: Option<Piece>) -> Option<Piece> {
    let p = p?;
    match p {
        Piece::Tam2 => Some(p),
        Piece::NonTam2Piece { prof, color, side } => Some(Piece::NonTam2Piece {
            prof,
            color,
            side: !side,
        }),
    }
}

/// [row, col]
pub type Coord = [usize; 2];

/// Serializes [`Coord`](./type.Coord.html) in JSON-style.
/// # Examples
/// ```
/// use cetkaik_core::*;
/// use cetkaik_core::relative::*;
///
/// assert_eq!(serialize_coord([5,6]), "[5,6]")
/// ```
#[must_use]
pub fn serialize_coord(coord: Coord) -> String {
    format!("[{},{}]", coord[0], coord[1])
}

#[must_use]
pub fn rotate_coord(c: Coord) -> Coord {
    [(8 - c[0]), (8 - c[1])]
}

#[must_use]
pub fn is_water([row, col]: Coord) -> bool {
    (row == 4 && col == 2)
        || (row == 4 && col == 3)
        || (row == 4 && col == 4)
        || (row == 4 && col == 5)
        || (row == 4 && col == 6)
        || (row == 2 && col == 4)
        || (row == 3 && col == 4)
        || (row == 5 && col == 4)
        || (row == 6 && col == 4)
}

fn serialize_side(side: Side) -> &'static str {
    match side {
        Side::Upward => "↑",
        Side::Downward => "↓",
    }
}

/// Serializes [`Piece`](./enum.Piece.html).
/// # Examples
/// ```
/// use cetkaik_core::*;
/// use cetkaik_core::relative::*;
///
/// assert_eq!(serialize_piece(Piece::Tam2), "皇");
/// assert_eq!(serialize_piece(Piece::NonTam2Piece {
///     prof: Profession::Uai1,
///     color: Color::Kok1,
///     side: Side::Downward
/// }), "赤将↓");
/// ```
#[must_use]
pub fn serialize_piece(p: Piece) -> String {
    match p {
        Piece::Tam2 => "皇".to_string(),
        Piece::NonTam2Piece { prof, color, side } => format!(
            "{}{}{}",
            super::serialize_color(color),
            super::serialize_prof(prof),
            serialize_side(side)
        ),
    }
}
