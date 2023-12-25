use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::enemy::Enemy;
use crate::CollisionSet;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::default())
            .add_systems(Update, (update_grid, enemy_collision.in_set(CollisionSet)));
    }
}

#[derive(Resource)]
struct Grid {
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}

fn update_grid(mut grid: ResMut<Grid>, query: Query<(Entity, &Transform), With<Enemy>>) {
    grid.cells.clear();
    for (entity, transform) in query.iter() {
        let cell = (
            (transform.translation.x / 300.) as i32,
            (transform.translation.y / 300.) as i32,
        );
        grid.cells.entry(cell).or_default().push(entity);
    }
}

fn enemy_collision(grid: Res<Grid>, mut transforms: Query<&mut Transform, With<Enemy>>) {
    for (_, entities) in &grid.cells {
        for (i, &entity1) in entities.iter().enumerate() {
            let direction = {
                let transform1 = transforms.get(entity1).unwrap();
                entities[i + 1..]
                    .iter()
                    .filter_map(|&entity2| {
                        let transform2 = transforms.get(entity2).unwrap();
                        let distance = Vec2::new(
                            transform1.translation.x - transform2.translation.x,
                            transform1.translation.y - transform2.translation.y,
                        );
                        if distance.length() <= 32. {
                            Some(distance.normalize())
                        } else {
                            None
                        }
                    })
                    .next()
            };
            if let Some(direction) = direction {
                let mut transform1 = transforms.get_mut(entity1).unwrap();
                transform1.translation.x += direction.x * 2.;
                transform1.translation.y += direction.y * 2.;
            }
        }
    }
}
