use crate::ScreenUnit;

#[derive(Copy, Clone, serde::Deserialize)]
pub struct Rectangle {
    x: ScreenUnit,
    y: ScreenUnit,
    width: ScreenUnit,
    height: ScreenUnit,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { x: 0.into(), y: 0.into(), width: 1.into(), height: 1.into() }
    }
}

impl Rectangle {
    pub fn at_x(mut self, x: impl Into<ScreenUnit>) -> Self {
        self.x = x.into();
        self
    }

    pub fn at_y(mut self, y: impl Into<ScreenUnit>) -> Self {
        self.y = y.into();
        self
    }

    pub fn height(&self) -> ScreenUnit {
        self.height
    }

    pub fn into_screen(
        self,
        total_lines: u16,
        total_columns: u16,
    ) -> ScreenRectangle {
        ScreenRectangle {
            x: self.x.to_cells(total_columns),
            y: self.y.to_cells(total_lines),
            width: self.width.to_cells(total_columns),
            height: self.height.to_cells(total_lines),
        }
    }

    pub fn x(&self) -> ScreenUnit {
        self.x
    }

    pub fn y(&self) -> ScreenUnit {
        self.y
    }

    pub fn width(&self) -> ScreenUnit {
        self.width
    }

    pub fn with_height(mut self, height: impl Into<ScreenUnit>) -> Self {
        self.height = height.into();
        self
    }

    pub fn with_width(mut self, width: impl Into<ScreenUnit>) -> Self {
        self.width = width.into();
        self
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
pub struct ScreenRectangle {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl ScreenRectangle {
    /// TODO: docs
    pub fn shrink_horizontally(mut self, by: u16) -> Self {
        self.width = self.width.saturating_sub(by);
        self
    }

    /// TODO: docs
    pub fn shrink_vertically(mut self, by: u16) -> Self {
        self.width = self.width.saturating_sub(by);
        self
    }

    /// TODO: docs
    pub fn split_horizontally(self, at: u16) -> (Self, Self) {
        let left_width = at;
        let right_width = self.width - left_width;
        let left = Self { width: left_width, ..self };
        let right = Self { width: right_width, x: left_width, ..self };
        (left, right)
    }

    /// TODO: docs
    pub fn split_vertically(self, at: u16) -> (Self, Self) {
        let top_height = at;
        let bottom_height = self.height - top_height;
        let top = Self { height: top_height, ..self };
        let bottom =
            Self { height: bottom_height, y: self.y + top_height, ..self };
        (top, bottom)
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn move_down(&mut self, by: u16) -> &mut Self {
        self.y += by;
        self
    }

    pub fn move_right(&mut self, by: u16) -> &mut Self {
        self.x += by;
        self
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}
