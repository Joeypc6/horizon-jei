fn jei_panel_tint(game: &Game<'_>, batch: &mut HudBatch) {
    jei_optional::if_kit_jei!("jei", {
    crate::game::hz_kit_window_jei::tint(game, batch);
} else { () })
}
