use crate::prelude::*;

/**
    Verifies that no branching moveset is done.
**/
#[derive(Copy, Clone)]
pub struct NoBranching {
    pub min_timeline: Layer,
    pub max_timeline: Layer,
}

impl NoBranching {
    pub fn new(info: &Info) -> Self {
        NoBranching {
            min_timeline: info.min_timeline(),
            max_timeline: info.max_timeline(),
        }
    }
}

impl Goal for NoBranching {
    #[inline]
    fn verify<'b>(
        &self,
        _path: &'b [Moveset],
        _game: &'b Game,
        partial_game: &'b PartialGame<'b>,
        _max_depth: Option<usize>,
    ) -> Option<bool> {
        Some(
            partial_game.info.min_timeline() == self.min_timeline
                && partial_game.info.max_timeline() == self.max_timeline,
        )
    }
}

/**
    Verifies that no branching moveset is done.

    **Note:** this filter loses accuracy.
**/
#[derive(Copy, Clone)]
pub struct MaxBranching {
    pub min_timeline: Layer,
    pub max_timeline: Layer,

    pub max_branches: usize,
}

impl MaxBranching {
    pub fn new(info: &Info, max_branches: usize) -> Self {
        MaxBranching {
            min_timeline: info.min_timeline(),
            max_timeline: info.max_timeline(),

            max_branches,
        }
    }
}

impl Goal for MaxBranching {
    #[inline]
    fn verify<'b>(
        &self,
        _path: &'b [Moveset],
        _game: &'b Game,
        partial_game: &'b PartialGame<'b>,
        _max_depth: Option<usize>,
    ) -> Option<bool> {
        let branches = self.min_timeline - partial_game.info.min_timeline()
            + partial_game.info.max_timeline()
            - self.max_timeline;
        Some(branches as usize <= self.max_branches)
    }
}

/**
    Ignores lines where an inefficient branching move occurs; does so by trimming lines where a branching move occured N moves ago.
    Useful in variants where going back in time is limited.
    You should give `depth` an **odd** value for the best results.

    TODO: flag in Moveset to indicate that a time travel was forced? (how would you even compute that?)

    **Note:** this filter loses accuracy.
**/
#[derive(Copy, Clone)]
pub struct InefficientBranching {
    pub depth: usize,
}

impl InefficientBranching {
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }
}

impl Goal for InefficientBranching {
    #[inline]
    fn verify<'b>(
        &self,
        path: &'b [Moveset],
        _game: &'b Game,
        _partial_game: &'b PartialGame<'b>,
        _max_depth: Option<usize>,
    ) -> Option<bool> {
        if path.len() >= self.depth {
            let ms = &path[path.len() - self.depth];
            Some(!ms.branches || ms.necessary_branching)
        } else {
            Some(true)
        }
    }
}

/**
    Ignore lines where a branching occurred before the DFS depth was down to `depth`, useful for use with IDDFS.

    **Note:** this filter loses accuracy.
**/
#[derive(Copy, Clone)]
pub struct BranchBefore {
    pub depth: usize,
}

impl BranchBefore {
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }
}

impl Goal for BranchBefore {
    #[inline]
    fn verify<'b>(
        &self,
        path: &'b [Moveset],
        _game: &'b Game,
        _partial_game: &'b PartialGame<'b>,
        max_depth: Option<usize>,
    ) -> Option<bool> {
        if let Some(max_depth) = max_depth {
            if max_depth - path.len() > self.depth {
                if let Some(ms) = path.last() {
                    return Some(!ms.branches || ms.necessary_branching)
                }
            }
        }
        Some(true)
    }
}

/**
    Ignores lines where an inactive timeline is created, unless said timeline is created with a king (adjustable).

    **Note:** this filter loses more accuracy than `InefficientBranching`, with little returns.
**/
#[derive(Copy, Clone)]
pub struct InactiveTimeline {
    pub allow_exile: bool,
    pub allow_dead_timelines: bool, // TODO: implement
}

impl InactiveTimeline {
    pub fn new(allow_exile: bool, allow_dead_timelines: bool) -> Self {
        Self {
            allow_exile,
            allow_dead_timelines,
        }
    }
}

impl Default for InactiveTimeline {
    fn default() -> Self {
        Self {
            allow_exile: true,
            allow_dead_timelines: true,
        }
    }
}

impl Goal for InactiveTimeline {
    #[inline]
    fn verify<'b>(
        &self,
        _path: &'b [Moveset],
        game: &'b Game,
        partial_game: &'b PartialGame<'b>,
        _max_depth: Option<usize>,
    ) -> Option<bool> {
        for tl in partial_game
            .info
            .timelines_white
            .iter()
            .chain(partial_game.info.timelines_black.iter())
        {
            if !partial_game.info.is_active(tl.index) {
                if self.allow_exile {
                    if let Some(from) = tl.starts_from {
                        if let (Some(board_from), Some(board_to)) = (
                            partial_game.get_board_with_game(game, from),
                            partial_game.get_board_with_game(game, (tl.index, from.1 + 1)),
                        ) {
                            let white = from.1 % 2 == 0;
                            let mut kings_from = 0;
                            let mut kings_to = 0;
                            for piece in board_from.pieces.iter() {
                                if let Tile::Piece(piece) = piece {
                                    kings_from += (piece.kind == PieceKind::King && piece.white == white) as usize;
                                }
                            }

                            for piece in board_to.pieces.iter() {
                                if let Tile::Piece(piece) = piece {
                                    kings_to += (piece.kind == PieceKind::King && piece.white == white) as usize;
                                }
                            }

                            if kings_from > kings_to {
                                return Some(true);
                            }
                        }
                    }
                }

                if self.allow_dead_timelines {
                    // TODO
                }

                return Some(false);
            }
        }
        Some(true)
    }
}
