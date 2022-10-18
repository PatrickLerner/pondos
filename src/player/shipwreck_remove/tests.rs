use super::*;
use crate::types::{Player, Ship, ShipSize};
use std::collections::HashMap;

fn setup() -> (World, Box<dyn Stage>) {
    let mut world = World::default();
    let mut resources = HashMap::new();
    resources.insert("some".to_owned(), 20);
    resources.insert("thing".to_owned(), 20);

    let player = Player {
        resources,
        convoy: vec![Ship::new(ShipSize::Small), Ship::new(ShipSize::Medium)],
        ..default()
    };
    world.insert_resource(Events::<PlayerShipwreckEvent>::default());
    world.insert_resource(player);

    let mut stage = SystemStage::parallel();
    stage.add_system(shipwreck_remove);

    (world, Box::new(stage))
}

#[test]
fn removes_ship() {
    let (mut world, mut stage) = setup();

    {
        let mut events = world
            .get_resource_mut::<Events<PlayerShipwreckEvent>>()
            .unwrap();
        events.send(PlayerShipwreckEvent { ship_index: 0 });
    }

    stage.run(&mut world);

    let player = world.get_resource::<Player>().unwrap();
    assert_eq!(player.convoy.len(), 1);
    let ship = player.convoy.first().unwrap();
    assert_eq!(ship.size, ShipSize::Medium);

    // removes resources proportionally
    // we lost a small ship and have a medium,
    // we lost 20/75 capacity, so ~10.6 -> 11 resources
    let total = player
        .resources
        .iter()
        .fold(0, |res, (_, amount)| res + amount);

    assert_eq!(total, 29);
}

#[test]
fn removes_multiple_ships() {
    let (mut world, mut stage) = setup();

    {
        let mut events = world
            .get_resource_mut::<Events<PlayerShipwreckEvent>>()
            .unwrap();
        events.send(PlayerShipwreckEvent { ship_index: 0 });
        events.send(PlayerShipwreckEvent { ship_index: 1 });
    }

    stage.run(&mut world);

    let player = world.get_resource::<Player>().unwrap();
    assert_eq!(player.convoy.len(), 0);

    // removes resources proportionally
    // we lost 70/75 capacity, so ~37.3 -> 38 resources
    let total = player
        .resources
        .iter()
        .fold(0, |res, (_, amount)| res + amount);

    assert_eq!(total, 2);
}
