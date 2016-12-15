use app::App;
use texture::Texture;
use transform::Transform;
use component::Component;
use component::Message;
use std::mem;

pub struct GameObject {
    pub transform : Transform,
    pub texture : Texture,
    // When searching for a component associated with a GameObject you will need to search both of
    // these vectors. At any given time either one of them could contain the Component you are
    // searching for. As messages are distributed to components they are migrated from
    // components to messaged_components and once the distribution of a message is complete
    // the two vectors are swapped.
    components : Vec<Box<Component>>, // These have not had a message sent (yet).
    messaged_components: Vec<Box<Component>>, // These have had a message sent.
}

impl GameObject {
    pub fn new(app : &mut App) -> GameObject {
        GameObject {
            transform : Transform::new(),
            components : vec!(),
            messaged_components : vec!(),
            texture : Texture::empty(app),
        }
    }

    pub fn update(&mut self, app : &mut App, delta_time : f64) {
        let mut pop_result = self.components.pop();
        while let Some(mut component) = pop_result {
            component.receive_message(app, self, &Message::Update{delta_time : delta_time});
            self.messaged_components.push(component);
            pop_result = self.components.pop();
        }
        assert_eq!(self.components.len(), 0);
        mem::swap(&mut self.components, &mut self.messaged_components);
    }

    pub fn add_component(&mut self, component : Box<Component>) {
        self.components.push(component);
    }
}
