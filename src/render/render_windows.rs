use novec::NoVec;
use glium::Rect;

// Stored with a box covering the whole screen being (-1, -1, 2, 2)
pub struct RenderWindow {
    pub left: f32,
    pub bottom: f32,
    pub width: f32,
    pub height: f32,
}

impl RenderWindow {
    pub fn new(left: f32, bottom: f32, width: f32, height: f32) -> RenderWindow {
        RenderWindow {
            left, 
            bottom,
            width,
            height,
        }
    }

    pub fn to_rect(&self, (x, y): (f32, f32)) -> Rect {
        let width = self.width * 0.5 * x;
        let height = self.height * 0.5 * y;
        let left = (self.left + 1.0) * 0.5 * x;
        let bottom = (-self.bottom + 1.0) * 0.5 * y;

        Rect {
            left: left as u32,
            bottom: bottom as u32,
            width: width as u32,
            height: height as u32,
        }
    }
}

pub struct RenderWindows {
    windows: NoVec<RenderWindow>,
    cached: Vec<Option<Rect>>,
}

impl RenderWindows {
    pub fn new(windows: impl Iterator<Item = RenderWindow>) -> RenderWindows {
        let mut owned_windows = NoVec::new();

        for item in windows {
            owned_windows.push(item);
        }

        RenderWindows {
            windows: owned_windows,
            cached: vec![],
        }
    }

    pub fn cache(&mut self, dims: (f32, f32)) {
        self.cached.clear();

        for (_, item) in self.windows.entries_iter() {
            self.cached.push(item.map(|x| x.to_rect(dims)));
        }
    }

    pub fn get_cached(&self, id: usize) -> Option<Rect> {
        if self.cached.len() > id {
            self.cached[id]
        }
        else {
            None
        }
    }
}