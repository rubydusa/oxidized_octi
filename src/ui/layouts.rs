use tui::layout::Rect;

pub struct VStackLayout {
    x: u16,
    y_offset: u16,
    width: u16,
    areas: Vec<Rect>,
}

impl VStackLayout {
    pub fn new(x: u16, y_offset: u16, width: u16) -> VStackLayout {
        VStackLayout {
            x,
            y_offset,
            width,
            areas: Vec::new(),
        }
    }

    pub fn push(&mut self, height: u16) {
        self.areas.push(Rect {
            x: self.x,
            y: self.y_offset,
            width: self.width,
            height,
        });

        self.y_offset += height;
    }

    pub fn margin(&mut self, margin: u16) {
        self.y_offset += margin;
    }

    pub fn layout(self) -> Vec<Rect> {
        self.areas
    }
}
