use crate::state::TilePos;

/// Tracks the state of a click-and-drag tile selection
#[derive(Debug, Clone, Default)]
pub struct DragSelection {
    /// Is a drag currently active?
    pub active: bool,
    /// Starting tile position (where mouse was first pressed)
    pub start_pos: Option<TilePos>,
    /// Current end tile position (where mouse is now)
    pub end_pos: Option<TilePos>,
}

impl DragSelection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin a new drag selection at the given position
    pub fn start(&mut self, pos: TilePos) {
        self.active = true;
        self.start_pos = Some(pos);
        self.end_pos = Some(pos);
    }

    /// Update the current end position of the drag
    pub fn update(&mut self, pos: TilePos) {
        if self.active {
            self.end_pos = Some(pos);
        }
    }

    /// Finish the drag and return the selection bounds (min, max)
    pub fn finish(&mut self) -> Option<(TilePos, TilePos)> {
        if !self.active {
            return None;
        }
        let result = self.get_selection_rect();
        self.cancel();
        result
    }

    /// Cancel the current drag selection
    pub fn cancel(&mut self) {
        self.active = false;
        self.start_pos = None;
        self.end_pos = None;
    }

    /// Get the normalized selection rectangle (min corner, max corner)
    pub fn get_selection_rect(&self) -> Option<(TilePos, TilePos)> {
        match (self.start_pos, self.end_pos) {
            (Some(start), Some(end)) => {
                let min_x = start.x.min(end.x);
                let max_x = start.x.max(end.x);
                let min_y = start.y.min(end.y);
                let max_y = start.y.max(end.y);
                Some((TilePos::new(min_x, min_y), TilePos::new(max_x, max_y)))
            }
            _ => None,
        }
    }
}
