use serde::{Serialize, Deserialize};
use crate::{battle::actions::Action, core::{pokemon::pokemon::Pokemon, pokemove::move_name::MoveName}};
use rand::{Rng, SeedableRng, rngs::StdRng};

#[derive(Clone, Serialize)]
pub struct BattleState {
    trainer_1_state: Option<SingleSideState>,
    trainer_2_state: Option<SingleSideState>,

    #[serde(skip)]
    rng: StdRng,
}

impl BattleState {
    pub fn new() -> Self {
        Self {
            trainer_1_state: None,
            trainer_2_state: None,
            rng: StdRng::from_entropy()
        }
    }

    pub fn get_side_mut(&mut self, trainer_1: bool) -> &mut SingleSideState {
        if trainer_1 {
            self.trainer_1_state.as_mut().unwrap()
        } else {
            self.trainer_2_state.as_mut().unwrap()
        }
    }

    pub fn deal_damage(&mut self, trainer_1_src: bool, damage: u32) {
        let side_state = self.get_side_mut(!trainer_1_src);
        side_state.take_damage(damage);
    }

    pub fn get_active_pokemon(&self, trainer_1: bool) -> &Pokemon {
        self.get_side(trainer_1).get_active_pokemon()
    }

    pub fn get_active_pokemon_mut(&mut self, trainer_1: bool) -> &mut Pokemon {
        self.get_side_mut(trainer_1).get_active_pokemon_mut()
    }

    pub fn get_move_for_action(&self, trainer_1: bool, action: &Action) -> Option<MoveName> {
        let pokemon = self.get_active_pokemon(trainer_1);
        pokemon.get_move_for_action(action)
    }

    pub fn get_move_for_move_action(&self, trainer_1: bool, action: &Action) -> MoveName {
        self.get_move_for_action(trainer_1, action).expect("Unexpected action for move")
    }

    fn get_side(&self, trainer_1: bool) -> &SingleSideState {
        if trainer_1 {
            self.trainer_1_state.as_ref().unwrap()
        } else {
            self.trainer_2_state.as_ref().unwrap()
        }
    }

    pub fn get_random_check(&mut self, numerator: u32, denominator: u32) -> bool {
        let rand_num = self.get_rand_num(denominator);
        rand_num < numerator
    }

    pub fn get_rand_num(&mut self, upper_bound: u32) -> u32 {
        self.rng.gen_range(0..upper_bound)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SingleSideState {
    active_pokemon_idx: usize,
    pokemon: Vec<Pokemon>,
}

impl SingleSideState {
    pub fn new(pokemon: Vec<Pokemon>) -> Self {
        Self {
            active_pokemon_idx: 0,
            pokemon,
        }
    }

    pub fn get_active_pokemon(&self) -> &Pokemon {
        &self.pokemon[self.active_pokemon_idx]
    }

    pub fn get_active_pokemon_mut(&mut self) -> &mut Pokemon {
        &mut self.pokemon[self.active_pokemon_idx]
    }

    // Returns true if the active Pokemon fainted
    pub fn take_damage(&mut self, damage: u32) -> bool {
        let active_pokemon = self.get_active_pokemon_mut();
        if damage >= active_pokemon.hp as u32 {
            active_pokemon.hp = 0;
            true
        } else {
            active_pokemon.hp -= damage as u16;
            false
        }
    }
}

