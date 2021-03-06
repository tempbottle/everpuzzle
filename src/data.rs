#[allow(unused)]
pub mod playfield_data {
    // columns of the grid 6 vertically
    pub const COLUMNS: usize = 6;

    // rows that are off the screen (used for garbage)
    pub const ROWS_INVISIBLE: usize = 12;

    // rows that can be seen 12 horizontally
    pub const ROWS_VISIBLE: usize = 12;

    // sum of rows
    pub const ROWS: usize = ROWS_INVISIBLE + ROWS_VISIBLE;

    // overall amount of blocks that will exist per stack
    pub const BLOCKS: usize = COLUMNS * ROWS;

    // -- Animation Times --

    // amount of frames the screen shakes by combo size
    // xCombo size                            4c, 5c, 6c, 7c
    pub const SHAKE_COMBO_TIME: [usize; 4] = [18, 18, 24, 42];

    // xChain size                            2x, 3x, 4x, 5, anything bigger will be set to 76
    pub const SHAKE_CHAIN_TIME: [usize; 4] = [42, 66, 66, 76];

    // Amount of frames you can stay alive while blocks are at the top
    pub const STOP_TIME: [u32; 10] = [121, 100, 80, 65, 50, 40, 30, 20, 10, 1];

    // CUSTOM Amount that the offsets will get incremented with each frame
    pub const RAISE_TIME: [f32; 10] = [0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.10];

    // CUSTOM Time that the Playfield will get stuck after each raise
    pub const RAISE_BLOCKED_TIME: u32 = 5;
}

// contains all important block animation times / lengths
#[allow(unused)]
pub mod block_data {
    // size of the block in pixels
    pub const BLOCK_SIZE: usize = 16;

    // TODO: Seperate possible Block colors into 2 arrays <30-11-18, Skytrias> //
    // pub const BLOCK_COLORS: [usize; [usize; ]] = [[0, 1, 2, 3, 4], [0, 1, 2, 3, 4, 5]]

    // the amount of colors that are available with each difficulty
    pub const NUMBER_COLORS_VS: [usize; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1];

    // time the block will stay in air before falling
    pub const HOVER_TIME: [u32; 10] = [12, 12, 11, 10, 9, 6, 5, 4, 3, 6];

    // time that each clearing pop will take
    pub const POP_TIME: [u32; 10] = [9, 9, 8, 8, 8, 8, 8, 7, 7, 7];

    // time that it takes to animate the flashing in clears
    pub const FLASH_TIME: [u32; 10] = [44, 44, 42, 42, 38, 36, 34, 32, 30, 28];

    // time the face of a block will be showing up for
    pub const FACE_TIME: [u32; 10] = [15, 14, 14, 13, 12, 11, 10, 10, 9, 8];

    // time a swap will take
    pub const SWAP_TIME: f32 = 5.0;

    // time the land will animate for
    pub const LAND_TIME: u32 = 10;
}

pub mod cursor_data {
    // This is a really hardcoded way to have actions but I couldn't
    // get anything working
    //
    // What I tried was using Strings instead with pushing the last num
    // to each action but I always got into issues
    //
    // all strings for their actions, up to 8 players possible
    pub const CURSOR_ACTIONS: [[&'static str; 6]; 8] = [
        ["up0", "down0", "left0", "right0", "swap0", "raise0"],
        ["up1", "down1", "left1", "right1", "swap1", "raise1"],
        ["up2", "down2", "left2", "right2", "swap2", "raise2"],
        ["up3", "down3", "left3", "right3", "swap3", "raise3"],
        ["up4", "down4", "left4", "right4", "swap4", "raise4"],
        ["up5", "down5", "left5", "right5", "swap5", "raise5"],
        ["up6", "down6", "left6", "right6", "swap6", "raise6"],
        ["up7", "down7", "left7", "right7", "swap7", "raise7"],
    ];
}
