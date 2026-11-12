fn jei_hud_before(game: &mut crate::game::Game<'_>) {
    jei_optional::if_kit_jei!("jei", {
    // An older JEI (kit before 15) still runs its own window: closed, so only this one shows.
    jei_optional::if_jei_stale!("jei", { crate::game::hz_kit_window_jei::close() } else { () });
    crate::game::hz_kit_window_jei::hud_before(game);
} else { let _ = game; () })
}
