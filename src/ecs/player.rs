use specs::prelude::*;


struct PlayerComponent {

}

impl PlayerComponent {
    
}

impl Component for PlayerComponent {
    type Storage = HashMapStorage<Self>;
}


struct PlayerSystem;

impl System<'a> for PlayerSystem {
    type SystemData = 
    (
        ReadStorage<'a, Position>, 
        ReadStorage<'a, Name>
    );

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::SystemData) {
        //let (pos, name) = data;

        // for (pos, name) in (&pos, &name).join() {
        //     println!("{:?} is at {:?}", &name.name, &pos);
        // }
    }
}