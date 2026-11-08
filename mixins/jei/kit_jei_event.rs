fn ae2_jei_event(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    event: &crate::game::native_hooks::NativePanelEvent,
) {
    jei_optional::if_kit_jei!("jei", {
    crate::game::hz_kit_layer_jei::event(game, mod_id, event);
} else { () })
}
