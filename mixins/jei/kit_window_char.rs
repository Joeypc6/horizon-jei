fn jei_window_char(game: &crate::game::Game<'_>, c: char, canceled: &mut bool) {
    jei_optional::if_kit_jei!("jei", {
    if game.player.is_none() || *canceled {
        return;
    }
    if crate::game::hz_kit_window_jei::is_open() && !c.is_control() {
        // Typed into the panel's search box when it has the focus; the game's own hook hands it over.
    }
    crate::game::hz_kit_creative_jei::char_typed(c, canceled);
} else { () })
}
