fn jei_hud_before(game: &mut crate::game::Game<'_>) {
    jei_optional::if_kit_jei!("jei", {
    crate::game::hz_kit_window_jei::hud_before(game);
} else { () })
}
