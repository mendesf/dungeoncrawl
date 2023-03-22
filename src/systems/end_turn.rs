use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Carried)]
#[read_component(AmuletOfYala)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] map: &Map,
) {
    let mut player = <(Entity, &Health, &Point)>::query()
        .filter(component::<Player>());
    let amulet = <&Carried>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .next();

    let current_state = *turn_state;
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };
    player.iter(ecs).for_each(|(player_entity, hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
            return;
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
            return;
        }
        if let Some(carried) = amulet {
            let Carried(entity) = carried;
            if player_entity == entity {
                new_state = TurnState::Victory;
            }
        }
    });
    *turn_state = new_state;
}