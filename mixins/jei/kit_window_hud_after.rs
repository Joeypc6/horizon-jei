fn jei_hud_after(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    jei_optional::if_kit_jei!("jei", {
    crate::game::hz_kit_window_jei::hud_after(game, batch);
} else { let _ = (game, batch); () })
}
