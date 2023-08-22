
use crate::world::{Allegiance, Damage, DamageText, Stocks, StocksText};
use bevy::log;
use bevy::prelude::*;
use std::vec::Vec;

const PLAYER_COUNT: usize = 1;

pub fn update_stocks(
    stocks_query: Query<(&Allegiance, &Stocks)>,
    mut text_query: Query<(&Allegiance, &mut Text), With<StocksText>>,
) {
    log::debug!("Updating stocks in UI");
    let mut stocks_vec: Vec<Option<&Stocks>> = vec![None; PLAYER_COUNT];
    for (allegiance, stocks) in stocks_query.iter() {
        stocks_vec[allegiance.handle.0 as usize] = Some(stocks);
    }
    for (allegiance, mut text) in text_query.iter_mut() {
        let num_stocks = stocks_vec[allegiance.handle.0]
            .unwrap_or_else(|| &Stocks { count: 0 })
            .count;
        log::trace!("Now has {num_stocks} stocks");
        text.sections[0].value = format!("{num_stocks} stocks");
    }
}

pub fn update_dmg(
    dmg_query: Query<(&Allegiance, &Damage)>,
    mut text_query: Query<(&Allegiance, &mut Text), With<DamageText>>,
) {
    log::debug!("Updating damage in UI");
    let mut damage_vec: Vec<Option<&Damage>> = vec![None; PLAYER_COUNT];
    for (allegiance, damage) in dmg_query.iter() {
        damage_vec[allegiance.handle.0 as usize] = Some(damage);
    }
    for (allegiance, mut text) in text_query.iter_mut() {
        let amount_dmg = damage_vec[allegiance.handle.0]
            .unwrap_or_else(|| &Damage { percent: 0 })
            .percent;
        log::trace!("Now has {amount_dmg} damage");
        text.sections[0].value = format!("{amount_dmg}%");
    }
}
