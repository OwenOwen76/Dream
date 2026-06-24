pub const MAIN_TILEMAP: TilemapDefinition = TilemapDefinition {
    tile_width: 16,
    tile_height: 16,
    atlas_width: 128,
    atlas_height: 240,
    sprites: &[
        // --- DIRT ---
        // Centers
        TilemapSprite {
            name: "dirt_1",
            pixel_x: 96,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_2",
            pixel_x: 112,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_3",
            pixel_x: 96,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_4",
            pixel_x: 112,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_5",
            pixel_x: 96,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_6",
            pixel_x: 112,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        // Dirt sides
        TilemapSprite {
            name: "dirt_side_l",
            pixel_x: 48,
            pixel_y: 64,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_side_r",
            pixel_x: 80,
            pixel_y: 64,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_side_t",
            pixel_x: 64,
            pixel_y: 48,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_side_b",
            pixel_x: 64,
            pixel_y: 80,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_corner_tl",
            pixel_x: 48,
            pixel_y: 48,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_corner_tr",
            pixel_x: 80,
            pixel_y: 48,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_corner_bl",
            pixel_x: 48,
            pixel_y: 80,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_corner_br",
            pixel_x: 80,
            pixel_y: 80,
            walkable: Walkable::Walkable,
        },
        // Dirt Inner Corners (Smallest path turns)
        TilemapSprite {
            name: "dirt_inner_tl",
            pixel_x: 48,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_inner_tr",
            pixel_x: 64,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_inner_bl",
            pixel_x: 48,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_inner_br",
            pixel_x: 64,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        // Dirt Cliff/Bottom Edges
        TilemapSprite {
            name: "dirt_edge_l",
            pixel_x: 48,
            pixel_y: 96,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_edge_mid",
            pixel_x: 64,
            pixel_y: 96,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_edge_r",
            pixel_x: 80,
            pixel_y: 96,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "dirt_cliff_face",
            pixel_x: 64,
            pixel_y: 64,
            walkable: Walkable::Walkable,
        },
        // --- GRASS FAMILY ---
        // Main Grass Island
        TilemapSprite {
            name: "grass_tl",
            pixel_x: 0,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_tc",
            pixel_x: 16,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_tr",
            pixel_x: 32,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_ml",
            pixel_x: 0,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_mc",
            pixel_x: 16,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_mr",
            pixel_x: 32,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_bl",
            pixel_x: 0,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_bc",
            pixel_x: 16,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_br",
            pixel_x: 32,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        // Grass Inner Corners
        TilemapSprite {
            name: "grass_inner_tl",
            pixel_x: 48,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_inner_tr",
            pixel_x: 64,
            pixel_y: 16,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_inner_bl",
            pixel_x: 48,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_inner_br",
            pixel_x: 64,
            pixel_y: 32,
            walkable: Walkable::Walkable,
        },
        // Grass Bottom Cliffs/Edges
        TilemapSprite {
            name: "grass_edge_l",
            pixel_x: 0,
            pixel_y: 96,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_edge_mid",
            pixel_x: 16,
            pixel_y: 96,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_edge_r",
            pixel_x: 32,
            pixel_y: 96,
            walkable: Walkable::Walkable,
        },
        // Cliff Grass corners
        TilemapSprite {
            name: "cliff_grass_tl",
            pixel_x: 80,
            pixel_y: 64,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "cliff_grass_tr",
            pixel_x: 96,
            pixel_y: 64,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "cliff_grass_bl",
            pixel_x: 80,
            pixel_y: 80,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "cliff_grass_br",
            pixel_x: 96,
            pixel_y: 80,
            walkable: Walkable::Walkable,
        },
        // Grass Variety
        TilemapSprite {
            name: "grass_1",
            pixel_x: 80,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_2",
            pixel_x: 96,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        TilemapSprite {
            name: "grass_3",
            pixel_x: 112,
            pixel_y: 0,
            walkable: Walkable::Walkable,
        },
        // --- WATER FAMILY ---
        // --- WATER-GRASS ---
        TilemapSprite {
            name: "water_grass_tl",
            pixel_x: 0,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_grass_tc",
            pixel_x: 16,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_grass_tr",
            pixel_x: 32,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_grass_ml",
            pixel_x: 0,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_mc",
            pixel_x: 16,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_grass_mr",
            pixel_x: 32,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_grass_bl",
            pixel_x: 0,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_grass_bc",
            pixel_x: 16,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_grass_br",
            pixel_x: 32,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        // --- WATER-DIRT ---
        TilemapSprite {
            name: "water_dirt_tl",
            pixel_x: 48,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_tc",
            pixel_x: 64,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_tr",
            pixel_x: 80,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_ml",
            pixel_x: 48,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_mc",
            pixel_x: 16,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_mr",
            pixel_x: 80,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_bl",
            pixel_x: 48,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_bc",
            pixel_x: 64,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_dirt_br",
            pixel_x: 80,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        // --- INNER-WATER ---
        TilemapSprite {
            name: "water_inner_tl",
            pixel_x: 96,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_inner_tr",
            pixel_x: 112,
            pixel_y: 160,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_inner_bl",
            pixel_x: 96,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_inner_br",
            pixel_x: 112,
            pixel_y: 176,
            walkable: Walkable::NonWalkable,
        },
        // --- NARROW 1x1 WATER  ---
        TilemapSprite {
            name: "water_narrow_v",
            pixel_x: 96,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_narrow_h",
            pixel_x: 112,
            pixel_y: 192,
            walkable: Walkable::NonWalkable,
        },
        // --- WATER w/ DECOR  ---
        TilemapSprite {
            name: "water_lily_flower",
            pixel_x: 112,
            pixel_y: 208,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_lily_plain",
            pixel_x: 112,
            pixel_y: 224,
            walkable: Walkable::NonWalkable,
        },
        TilemapSprite {
            name: "water_sparkle",
            pixel_x: 112,
            pixel_y: 240,
            walkable: Walkable::NonWalkable,
        },
    ],
};
