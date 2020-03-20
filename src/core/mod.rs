pub struct Engine {
    dimensions: (u32, u32),
    title: String,

    scene_list: Vec<()>,
    scene_idx: usize,
}

pub struct Igniter {
    // window-related stuff
    dimensions: Option<(u32, u32)>,
    title: Option<String>,
    
    // scene
    scene_list: Vec<()>,
    starting_scene: usize,
}

impl Igniter {
    pub fn ignite() -> Self {
        Igniter {
            dimensions: None,
            title: None,
            scene_list: vec![],
            starting_scene: 0,
        }
    }

    pub fn with_dimensions(mut self, dim: (u32, u32)) -> Self {
        self.dimensions.replace(dim);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title.replace(title);
        self
    }

    pub fn add_scene(mut self, scene: ()) -> Self {
        self.scene_list.push(scene);
        self
    }

    pub fn starting_scene(mut self, idx: usize) -> Self {
        self.starting_scene = idx;
        self
    }

    pub fn combust(self) -> Engine {
        Engine {
            dimensions: self.dimensions.unwrap_or((1280, 720)),
            title: self.title.unwrap_or("Snowfire".to_string()),
            scene_list: self.scene_list,
            scene_idx: self.starting_scene
        }
    }
}

impl Engine {
    pub fn burn(&self) {
        println!("oof ouch ouie it hurts oof my bones ow oof");
    }
}
