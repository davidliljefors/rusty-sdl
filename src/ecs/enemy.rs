use specs::prelude::*;


struct EnemyComponent {

    destination:Position,
    
}

impl EnemyComponent {
    
}

impl Component for EnemyComponent {
    type Storage = HashMapStorage<Self>;
}


struct EnemySystem;

impl System<'a> for EnemySystem {
    type SystemData = 
    (
        ReadStorage<'a, Position>,
        ReadStorage<'a, PlayerComponent>,
        WriteStorage<'a, EnemyComponent>,
        WriteStorage<'a, Velocity>,
    );

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::SystemData) {
        
        let (position_storage, player_component, mut enemy_component) = data;

        let target_position:Position;
        for (player_pos, enemy_compoent) in (&position_storage, &player_component).join() {
            let target_position = player_pos;
        }

        for (enemy_pos, mut enemy_compoent) in (&position_storage, &mut enemy_component).join() {
            
            let distance = Position::distance(enemy_pos, enemy_component.destination);
            if distance < 5.0 {
                enemy_component.destination = target_position;
            }
                        
        }
    }
}