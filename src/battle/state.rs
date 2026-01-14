use crate::{
    battle::{actions::Action, pokemon_battle_instance::PokemonBattleInstance},
    core::{pokemon::pokemon::Pokemon, pokemove::move_name::MoveName},
};
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

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
            rng: StdRng::from_entropy(),
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

    pub fn get_active_pokemon(&self, trainer_1: bool) -> &PokemonBattleInstance {
        self.get_side(trainer_1).get_active_pokemon()
    }

    pub fn get_active_pokemon_mut(&mut self, trainer_1: bool) -> &mut PokemonBattleInstance {
        self.get_side_mut(trainer_1).get_active_pokemon_mut()
    }

    pub fn get_move_for_action(&self, trainer_1: bool, action: &Action) -> Option<MoveName> {
        let pokemon = self.get_active_pokemon(trainer_1);
        pokemon.pokemon.get_move_for_action(action)
    }

    pub fn get_move_for_move_action(&self, trainer_1: bool, action: &Action) -> MoveName {
        self.get_move_for_action(trainer_1, action)
            .expect("Unexpected action for move")
    }

    pub fn get_side(&self, trainer_1: bool) -> &SingleSideState {
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

    pub fn get_rand_num_inclusive(&mut self, lower_bound: u8, upper_bound: u8) -> u8 {
        self.rng.gen_range(lower_bound..=upper_bound)
    }
}

#[derive(Clone, Serialize)]
pub struct SingleSideState {
    active_pokemon_idx: usize,
    pokemon: Vec<PokemonBattleInstance>,
    trainer_1: bool,
}

impl SingleSideState {
    pub fn new(pokemon: Vec<Pokemon>, trainer_1: bool) -> Self {
        Self {
            active_pokemon_idx: 0,
            pokemon: pokemon
                .iter()
                .map(|p| PokemonBattleInstance::new(p.clone(), trainer_1))
                .collect(),
            trainer_1,
        }
    }

    pub fn get_active_pokemon(&self) -> &PokemonBattleInstance {
        &self.pokemon[self.active_pokemon_idx]
    }

    pub fn get_active_pokemon_mut(&mut self) -> &mut PokemonBattleInstance {
        &mut self.pokemon[self.active_pokemon_idx]
    }

    pub fn set_active_pokemon(&mut self, idx: usize) -> &PokemonBattleInstance {
        self.active_pokemon_idx = idx;
        self.get_active_pokemon()
    }

    // Returns true if the active Pokemon fainted
    pub fn take_damage(&mut self, damage: u32) -> bool {
        let active_pokemon = self.get_active_pokemon_mut();
        if damage >= active_pokemon.pokemon.hp as u32 {
            active_pokemon.pokemon.hp = 0;
            true
        } else {
            active_pokemon.pokemon.hp -= damage as u16;
            false
        }
    }

    pub fn out_of_usable_pokemon(&self) -> bool {
        self.pokemon.iter().all(|p| p.is_fainted())
    }
}
