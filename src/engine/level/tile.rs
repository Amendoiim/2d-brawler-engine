//! Tile-based environment system

use glam::Vec2;
use serde::{Deserialize, Serialize};

/// A single tile in the level grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tile {
    /// Empty space (air)
    Empty,
    /// Floor tile (walkable)
    Floor,
    /// Wall tile (solid, not walkable)
    Wall,
    /// Door tile (can be opened/closed)
    Door,
    /// Water tile (walkable but with effects)
    Water,
    /// Lava tile (damaging)
    Lava,
    /// Grass tile (decorative floor)
    Grass,
    /// Stone tile (decorative floor)
    Stone,
    /// Wood tile (decorative floor)
    Wood,
    /// Metal tile (decorative floor)
    Metal,
    /// Sand tile (decorative floor)
    Sand,
    /// Snow tile (decorative floor)
    Snow,
    /// Ice tile (slippery floor)
    Ice,
    /// Spikes tile (damaging floor)
    Spikes,
    /// Teleporter tile (special)
    Teleporter,
    /// Secret door tile (hidden)
    SecretDoor,
    /// Window tile (decorative wall)
    Window,
    /// Pillar tile (decorative obstacle)
    Pillar,
}

/// Tile properties and behavior
#[derive(Debug, Clone, Copy)]
pub struct TileProperties {
    /// Whether the tile is walkable
    pub walkable: bool,
    /// Whether the tile is solid (blocks movement)
    pub solid: bool,
    /// Whether the tile is transparent (can see through)
    pub transparent: bool,
    /// Whether the tile damages entities
    pub damaging: bool,
    /// Damage amount per frame
    pub damage: f32,
    /// Whether the tile is slippery
    pub slippery: bool,
    /// Friction coefficient (0.0 = no friction, 1.0 = full friction)
    pub friction: f32,
    /// Whether the tile is interactive
    pub interactive: bool,
    /// Whether the tile is destructible
    pub destructible: bool,
    /// Health of the tile (if destructible)
    pub health: f32,
    /// Texture ID for rendering
    pub texture_id: u32,
    /// Color tint for the tile
    pub color: [f32; 4],
}

impl Tile {
    /// Get the properties for this tile type
    pub fn properties(&self) -> TileProperties {
        match self {
            Tile::Empty => TileProperties {
                walkable: true,
                solid: false,
                transparent: true,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 0,
                color: [0.0, 0.0, 0.0, 0.0], // Transparent
            },
            Tile::Floor => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1000,
                color: [1.0, 1.0, 1.0, 1.0],
            },
            Tile::Wall => TileProperties {
                walkable: false,
                solid: true,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: true,
                health: 100.0,
                texture_id: 1001,
                color: [0.5, 0.5, 0.5, 1.0],
            },
            Tile::Door => TileProperties {
                walkable: false,
                solid: true,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: true,
                destructible: true,
                health: 50.0,
                texture_id: 1002,
                color: [0.8, 0.6, 0.4, 1.0],
            },
            Tile::Water => TileProperties {
                walkable: true,
                solid: false,
                transparent: true,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 0.8,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1003,
                color: [0.2, 0.4, 0.8, 0.7],
            },
            Tile::Lava => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: true,
                damage: 10.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1004,
                color: [1.0, 0.2, 0.0, 1.0],
            },
            Tile::Grass => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1005,
                color: [0.2, 0.8, 0.2, 1.0],
            },
            Tile::Stone => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1006,
                color: [0.6, 0.6, 0.6, 1.0],
            },
            Tile::Wood => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: true,
                health: 25.0,
                texture_id: 1007,
                color: [0.6, 0.4, 0.2, 1.0],
            },
            Tile::Metal => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: true,
                health: 200.0,
                texture_id: 1008,
                color: [0.7, 0.7, 0.8, 1.0],
            },
            Tile::Sand => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 0.9,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1009,
                color: [0.9, 0.8, 0.6, 1.0],
            },
            Tile::Snow => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: true,
                friction: 0.7,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1010,
                color: [0.9, 0.9, 1.0, 1.0],
            },
            Tile::Ice => TileProperties {
                walkable: true,
                solid: false,
                transparent: true,
                damaging: false,
                damage: 0.0,
                slippery: true,
                friction: 0.3,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1011,
                color: [0.8, 0.9, 1.0, 0.8],
            },
            Tile::Spikes => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: true,
                damage: 5.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: false,
                health: 0.0,
                texture_id: 1012,
                color: [0.8, 0.8, 0.8, 1.0],
            },
            Tile::Teleporter => TileProperties {
                walkable: true,
                solid: false,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: true,
                destructible: false,
                health: 0.0,
                texture_id: 1013,
                color: [0.8, 0.2, 0.8, 1.0],
            },
            Tile::SecretDoor => TileProperties {
                walkable: false,
                solid: true,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: true,
                destructible: true,
                health: 200.0,
                texture_id: 1014,
                color: [0.4, 0.4, 0.4, 1.0],
            },
            Tile::Window => TileProperties {
                walkable: false,
                solid: true,
                transparent: true,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: true,
                health: 10.0,
                texture_id: 1015,
                color: [0.7, 0.8, 1.0, 0.5],
            },
            Tile::Pillar => TileProperties {
                walkable: false,
                solid: true,
                transparent: false,
                damaging: false,
                damage: 0.0,
                slippery: false,
                friction: 1.0,
                interactive: false,
                destructible: true,
                health: 150.0,
                texture_id: 1016,
                color: [0.6, 0.5, 0.4, 1.0],
            },
        }
    }

    /// Check if this tile is walkable
    pub fn is_walkable(&self) -> bool {
        self.properties().walkable
    }

    /// Check if this tile is solid
    pub fn is_solid(&self) -> bool {
        self.properties().solid
    }

    /// Check if this tile is transparent
    pub fn is_transparent(&self) -> bool {
        self.properties().transparent
    }

    /// Check if this tile is damaging
    pub fn is_damaging(&self) -> bool {
        self.properties().damaging
    }

    /// Get the damage amount for this tile
    pub fn damage(&self) -> f32 {
        self.properties().damage
    }

    /// Check if this tile is slippery
    pub fn is_slippery(&self) -> bool {
        self.properties().slippery
    }

    /// Get the friction coefficient for this tile
    pub fn friction(&self) -> f32 {
        self.properties().friction
    }

    /// Check if this tile is interactive
    pub fn is_interactive(&self) -> bool {
        self.properties().interactive
    }

    /// Check if this tile is destructible
    pub fn is_destructible(&self) -> bool {
        self.properties().destructible
    }

    /// Get the health of this tile
    pub fn health(&self) -> f32 {
        self.properties().health
    }

    /// Get the texture ID for this tile
    pub fn texture_id(&self) -> u32 {
        self.properties().texture_id
    }

    /// Get the color for this tile
    pub fn color(&self) -> [f32; 4] {
        self.properties().color
    }
}

/// Tile map for managing a grid of tiles
#[derive(Debug, Clone)]
pub struct TileMap {
    /// Width of the tile map
    pub width: u32,
    /// Height of the tile map
    pub height: u32,
    /// Grid of tiles
    pub tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Create a new tile map
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![Tile::Empty; height as usize]; width as usize],
        }
    }

    /// Get a tile at the specified coordinates
    pub fn get_tile(&self, x: u32, y: u32) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.tiles[x as usize][y as usize])
        } else {
            None
        }
    }

    /// Set a tile at the specified coordinates
    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) -> bool {
        if x < self.width && y < self.height {
            self.tiles[x as usize][y as usize] = tile;
            true
        } else {
            false
        }
    }

    /// Check if a position is walkable
    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.is_walkable()
        } else {
            false
        }
    }

    /// Check if a position is solid
    pub fn is_solid(&self, x: u32, y: u32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.is_solid()
        } else {
            true // Out of bounds is considered solid
        }
    }

    /// Get all tiles in a rectangular area
    pub fn get_tiles_in_area(&self, x: u32, y: u32, width: u32, height: u32) -> Vec<Vec<Tile>> {
        let mut result = Vec::new();
        for i in 0..width {
            let mut row = Vec::new();
            for j in 0..height {
                if let Some(tile) = self.get_tile(x + i, y + j) {
                    row.push(tile);
                } else {
                    row.push(Tile::Empty);
                }
            }
            result.push(row);
        }
        result
    }

    /// Set tiles in a rectangular area
    pub fn set_tiles_in_area(&mut self, x: u32, y: u32, width: u32, height: u32, tiles: Vec<Vec<Tile>>) {
        for i in 0..width.min(tiles.len() as u32) {
            for j in 0..height.min(tiles[i as usize].len() as u32) {
                self.set_tile(x + i, y + j, tiles[i as usize][j as usize]);
            }
        }
    }

    /// Find all tiles of a specific type
    pub fn find_tiles(&self, tile_type: Tile) -> Vec<(u32, u32)> {
        let mut positions = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.tiles[x as usize][y as usize] == tile_type {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    /// Count tiles of a specific type
    pub fn count_tiles(&self, tile_type: Tile) -> u32 {
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.tiles[x as usize][y as usize] == tile_type {
                    count += 1;
                }
            }
        }
        count
    }
}
