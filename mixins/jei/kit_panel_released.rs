fn jei_window_released(screen: &crate::game::native_hooks::NativeScreenView<'_>) {
    jei_optional::if_kit_jei!("jei", {
    crate::game::hz_kit_window_jei::release(screen.mouse);
} else { () })
}
