#![allow(unused_variables)]
use amethyst::ecs::{Component, DenseVecStorage};
use data::playfield_data::{BLOCKS, COLUMNS};
use rand::prelude::*;

// const STATIC_SEED: [u8; 16] = [0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

// resource that stores the rng generator that will be global
// accessed via the world
#[derive(Debug, Clone)]
pub struct KindGenerator {
    pub rng: SmallRng,
}

// generates a new seed to be used by kind_generators rng
pub fn generate_random_seed() -> [u8; 16] {
    let mut rand_seed: [u8; 16] = [0; 16];

    for x in &mut rand_seed {
        *x = rand::random::<u8>();
    }

    rand_seed
}

// returns a stack of blocks where no numbers are the same next to each other
// also nulls kinds randomly and creates holes throughout the stack
// also has zones in which all blocks will definitely be nulled
// and a safe zone where no nulling happens
impl KindGenerator {
    pub fn new(seed: [u8; 16]) -> KindGenerator {
        KindGenerator {
            rng: SmallRng::from_seed(seed),
        }
    }

    pub fn new_rng(&mut self, seed: [u8; 16]) {
        self.rng = SmallRng::from_seed(seed);
    }

    pub fn create_stack(&mut self, safe: usize, nulling: usize) -> Vec<i32> {
        let safe_zone: usize = safe * COLUMNS;
        let nulling_zone: usize = nulling * COLUMNS;

        // empty array to destined length
        let size: usize = BLOCKS; //TODO: ROWS_VIS data
        let mut nums: Vec<i32> = Vec::new();
        nums.resize(size, -1);

        // scoped previous number that saves the newest generated number
        let mut prev_before = -1;

        for i in 0..size {
            let mut new_num: i32 = 0;
            let mut bot_num: i32 = -1; // by default -1
            let mut skip: bool = false;

            // set bot_num once it respects the boundaries
            if i > COLUMNS {
                bot_num = nums[i - COLUMNS];

                // if bot_num is -1, just set new_num to -1 and skip
                if bot_num == -1 {
                    skip = true;
                    new_num = -1;
                }
            }

            if !skip {
                // when over start to go through
                if i != 0 {
                    // if the right wall is hit (after i * 6) then be true
                    if i % COLUMNS + 1 != 0 {
                        new_num = self.get_number_in_zone(
                            prev_before,
                            bot_num,
                            i,
                            safe_zone,
                            nulling_zone,
                        );
                    } else {
                        new_num = self.get_number_in_zone(-1, bot_num, i, safe_zone, nulling_zone);
                    }
                } else {
                    new_num = self.get_number_in_zone(-1, -1, i, safe_zone, nulling_zone);
                }
            }

            prev_before = new_num;
            nums[i] = new_num;
        }

        nums
    }

    // create rows defined by the size of the dimensions parameter,
    // the generated vector will not have any numbers that match neighboring
    // numbers
    pub fn create_rows(&mut self, dimensions: (usize, usize)) -> Vec<i32> {
        // empty array to destined length
        let size: usize = dimensions.0 * dimensions.1;
        let mut nums: Vec<i32> = Vec::new();
        nums.resize(size, -1);

        // scoped previous number that saves the newest generated number
        let mut prev_before = -1;

        for i in 0..size {
            let mut new_num;
            let mut bot_num: i32 = -1; // by default -1

            // set bot_num once it respects the boundaries
            if i > COLUMNS {
                bot_num = nums[i - COLUMNS];
            }

            // if the right wall is hit (after i * 6) then be true
            if i % COLUMNS + 1 != 0 {
                new_num = self.get_number(prev_before, bot_num);
            } else {
                new_num = self.get_number(-1, bot_num);
            }

            prev_before = new_num;
            nums[i] = new_num;
        }

        nums
    }

    // returns a randomly chosen number out of an array
    // you can erase contents inside by specifying them in the parameters
    // otherwise they'll remain available to be chosen randomly
    fn get_number_in_zone(
        &mut self,
        cond1: i32,
        cond2: i32,
        iterator: usize,
        safe_zone: usize,
        null_zone: usize,
    ) -> i32 {
        let mut numbers: Vec<i32> = vec![-1, 0, 1, 2, 3, 4];

        if iterator >= null_zone {
            return -1;
        }

        if safe_zone >= iterator {
            numbers.retain(|x| x != &-1); // leave everything but -1
        }

        if cond1 != -1 {
            numbers.retain(|x| x != &cond1);
        }

        if cond2 != -1 {
            numbers.retain(|x| x != &cond2);
        }

        return numbers[self.rng.gen_range(0, numbers.len())];
    }

    // returns a randomly chosen number out of an array
    // you can erase contents inside by specifying them in the parameters
    // otherwise they'll remain available to be chosen randomly
    fn get_number(&mut self, cond1: i32, cond2: i32) -> i32 {
        let mut numbers: Vec<i32> = vec![0, 1, 2, 3, 4];

        if cond1 != -1 {
            numbers.retain(|x| x != &cond1);
        }

        if cond2 != -1 {
            numbers.retain(|x| x != &cond2);
        }

        return numbers[self.rng.gen_range(0, numbers.len())];
    }
}

impl Component for KindGenerator {
    type Storage = DenseVecStorage<Self>;
}
