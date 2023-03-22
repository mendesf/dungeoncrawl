use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Damage)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );
    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, player)| (*entity, player.map_level))
        .next()
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut weapons_query = <(&Item, &Name, &Carried, &Damage)>::query()
        .filter(component::<Weapon>());
    let mut weapons_y = 3;
    weapons_query
        .iter(ecs)
        .filter(|(_, _, carried, _)| carried.0 == player)
        .for_each(|(_, name, _, damage)| {
            draw_batch.print(
                Point::new(3, weapons_y),
                format!("{} ({})", &name.0, &damage.0),
            );
            weapons_y += 1;
        });
    if weapons_y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Weapons:",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    let margin = if weapons_y > 3 { weapons_y + 1 } else { 2 };

    let mut consumables_query = <(&Item, &Name, &Carried)>::query()
        .filter(component::<Consumable>());
    let mut consumables_y = margin + 1;
    consumables_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(
                Point::new(3, consumables_y),
                format!("{} : {}", consumables_y - margin, &name.0),
            );
            consumables_y += 1;
        });
    if consumables_y > margin + 1 {
        draw_batch.print_color(
            Point::new(3, margin),
            "Consumables:",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}