pub struct LowHpTypeBoost {
    pub boosted_type: Type,
    pub multiplier: f32, // 1.5 for Gen 4
}

impl EventHandler for LowHpTypeBoost {
    fn subscriptions(&self) -> &'static [EventType] {
        // We only care about power modification
        static EVENTS: &[EventType] = &[
            EventType::ModifyMovePower {
                user: PokemonId::DUMMY,
                move_type: Type::Normal,
                power: 0,
            }
        ];
        EVENTS
    }

    fn handle(
        &mut self,
        event: &EventType,
        battle_state: &mut BattleState,
        _queue: &mut Vec<EventType>,
    ) {
        if let EventType::ModifyMovePower {
            user,
            move_type,
            power,
        } = event
        {
            // Must match the ability's type
            if *move_type != self.boosted_type {
                return;
            }

            let pokemon = battle_state.get_pokemon(*user);

            // Gen 4: HP ≤ 1/3
            if pokemon.current_hp * 3 <= pokemon.max_hp {
                let boosted = (*power as f32 * self.multiplier).floor() as i32;
                // SAFETY NOTE:
                // You will probably want EventType to be mutable in practice.
                // For now assume power is interior-mutable or stored in BattleState.
                battle_state.set_modified_power(*user, boosted);
            }
        }
    }

    fn priority(&self) -> i32 {
        // Same priority as Showdown: standard damage modifiers
        0
    }
}
