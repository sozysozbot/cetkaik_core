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
pub const fn rotate_coord(c: Coord) -> Coord {
    [(8 - c[0]), (8 - c[1])]
}

#[must_use]
pub const fn is_water([row, col]: Coord) -> bool {
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

const fn serialize_side(side: Side) -> &'static str {
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

pub type Board = [Row; 9];
pub type Row = [Option<Piece>; 9];

#[derive(Debug, Clone)]
pub struct Field {
    pub current_board: Board,
    pub hop1zuo1of_upward: Vec<NonTam2PieceUpward>,
    pub hop1zuo1of_downward: Vec<NonTam2PieceDownward>,
}

impl Field {
    pub fn insert_nontam_piece_into_hop1zuo1(
        &mut self,
        color: Color,
        prof: Profession,
        side: Side,
    ) {
        match side {
            Side::Upward => self
                .hop1zuo1of_upward
                .push(NonTam2PieceUpward { color, prof }),
            Side::Downward => self
                .hop1zuo1of_downward
                .push(NonTam2PieceDownward { color, prof }),
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
            Side::Upward => {
                let mut that = self.clone();
                let index = that
                    .hop1zuo1of_upward
                    .iter()
                    .position(|x| *x == NonTam2PieceUpward { color, prof })?;
                that.hop1zuo1of_upward.remove(index);
                Some(that)
            }
            Side::Downward => {
                let mut that = self.clone();
                let index = that
                    .hop1zuo1of_downward
                    .iter()
                    .position(|x| *x == NonTam2PieceDownward { color, prof })?;
                that.hop1zuo1of_downward.remove(index);
                Some(that)
            }
        }
    }
}

#[must_use]
pub fn rotate_board(b: Board) -> Board {
    let mut ans: Board = [
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
    ];
    for i in 0..9 {
        for j in 0..9 {
            ans[i][j] = rotate_piece_or_null(b[8 - i][8 - j]);
        }
    }
    ans
}
