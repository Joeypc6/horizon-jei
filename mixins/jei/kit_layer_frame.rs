fn kit_layer_frame_jei(game: &mut crate::game::Game<'_>) {
    jei_optional::if_kit_jei!("jei", {
    if game.native_on(crate::game::jei::ID) {
        crate::game::hz_kit_layer_jei::frame(game);
    }
} else { () })
}
