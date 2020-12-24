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
pub fn to_absolute_board(board: &relative::Board, p: Perspective) -> absolute::Board {
    let mut ans = std::collections::HashMap::new();
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piece) = board[i][j] {
                ans.insert(to_absolute_coord([i, j], p), to_absolute_piece(piece, p));
            }
        }
    }
    ans
}

#[must_use]
pub fn to_absolute_field(field: relative::Field, p: Perspective) -> absolute::Field {
    let relative::Field {
        hop1zuo1of_downward,
        hop1zuo1of_upward,
        current_board,
    } = field;
    absolute::Field {
        board: to_absolute_board(&current_board, p),
        ia_side_hop1zuo1: match p {
            Perspective::IaIsDown => hop1zuo1of_downward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceDownward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
            Perspective::IaIsUp => hop1zuo1of_upward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceUpward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
        },
        a_side_hop1zuo1: match p {
            Perspective::IaIsDown => hop1zuo1of_upward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceUpward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
            Perspective::IaIsUp => hop1zuo1of_downward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceDownward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
        },
    }
}

#[must_use]
pub const fn to_absolute_side(side: relative::Side, p: Perspective) -> absolute::Side {
    match (side, p) {
        (relative::Side::Upward, Perspective::IaIsDown)
        | (relative::Side::Downward, Perspective::IaIsUp) => absolute::Side::IASide,
        (relative::Side::Downward, Perspective::IaIsDown)
        | (relative::Side::Upward, Perspective::IaIsUp) => absolute::Side::ASide,
    }
}

#[must_use]
pub const fn to_absolute_piece(piece: relative::Piece, p: Perspective) -> absolute::Piece {
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
