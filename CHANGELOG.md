# Changelog

## Unreleased — The recipe window and the mod kit

This adds, inside JEI itself, what the Horizon ports of AE2, Jade, Elevator, Iron Furnaces and Hostile Neural
Networks each carried as a copy of their "JEI kit". Built and tested on Nightly `6560641`, with hook points 313.

- **The recipe window**: JEI's centred window over the screen, with a tab per category along the top, the stations
  each category is made on down the left, a title, a page bar and the recipes on cards.
- **Where items come from** (Just Enough Resources' pages): Mob Drops from the loot tables, World Gen for ores and
  other placed blocks, and the other sources a recipe table doesn't name.
- **The panel on the creative screen**, beside the creative inventory.
- **An API for mods**: `Provider` and `register` (`crate::game::hz_kit_layer_jei`) add a mod's own recipe tabs and
  the stations of its categories.
- **One copy for every mod**: the mods' kits (version 3 and later) see JEI's marker module `hz_jeikit_v<version>_jei`
  and leave their own recipe window, sources, panel and provider API out of the build, using JEI's.
  Their W guide (a port of AE2's LGPL guidebook) stays theirs and is not in this repository; the window reaches it
  only through `if_guide!`, so JEI on its own builds without it.
- Builds with JEI's own 0.2 viewer (`Viewer::with_variants`); the kit's macro crate `jei_optional` picks the API at
  compile time.
- **Kit 14, fluids as textures**: a recipe card's fluid slot always draws the fluid's still texture, tinted and tiled
  16 px at a time as JEI's fluid renderer does, filled from the bottom by amount over capacity. It looks the sprite
  up under the common mod paths (`<path>_still`, `fluid/<path>_still`, `fluid/<path>/still`, `<path>/still`) and
  draws the missing-texture square when there is none, never a bucket. A slot's `bucket` field is ignored. Built on
  Nightly `fdc87ae` (2026-11-08) with all ten ported mods and checked on Tinkers' casting, Ender IO's combustion
  generator and Powah's magmatic fluids.
- **Kit 15, any JEI version**: a mod uses JEI's copy of the kit only when JEI's marker version is at least the
  newest kit among the ports in the build; otherwise the newest port compiles its own copy, so mods no longer fail to
  compile next to an older JEI.

## 0.2.1 — World switch cache fix

A patch release fixing viewer cache persistence across world transitions when JEI is disabled:

- **Fix:** Clear cached viewer on world unload even while JEI is disabled. Previously, `sync` returned early when `native_on(jei::ID)` was false before the unload branch, leaving the old world's viewer in memory across world switches.
- **Session management:** Encapsulate viewer lifecycle into `Session` in `runtime.rs`, ensuring the cache is unconditionally cleared upon leaving a world.
- **Regression test:** Cover the disable -> switch worlds -> re-enable lifecycle sequence.

Verified with `cargo test` (44) and `native.py check` on Nightly `6560641`.

## 0.2.0 — Overlay and bookmarks

The overlay JEI is known for, on Horizon's screen hooks. It needs hook points
of version 313 or later (`native_item_stack`); it was tested on Nightly
2026-11-07, `6560641`.

- **Bookmarks.** The **A** key bookmarks the stack under the pointer, over a
  panel item or any slot of the container screen. The bar sits at the screen's
  top left corner, two columns of twelve: the left button opens a stack's
  recipes, the right one its uses, shift-click or the middle button takes the
  bookmark away, and dragging a slot moves it. Each slot shows the stack, and
  hovering it draws the item's name and its actions.
- **Kept between sessions.** `config/jei/bookmarks.txt` holds the bar and
  `config/jei/jei.cfg` the rest; `/jei config` says where they are.
- **Item tooltips** now say `R: Recipes (12)` and `U: Uses (3)`, and mark the
  bookmarked stack with a star. They go into every tooltip the game shows,
  the panel's included, and count exactly: a stack with components counts the
  recipes the tables name for those components.
- **Subtypes**: every `(item, components)` pair the recipe tables name is an
  item of its own in the list, so the potions and the tipped arrows are
  there like JEI's subtypes. Up to eight variants per item.
- **The list's order**: by name or by id, from the settings page or
  `/jei sort name|id`, kept in the config file.
- New commands and settings: `/jei sort`, `/jei bookmarks clear`, `/jei config`;
  the settings page's buttons now cover the order, the tooltip hints and the bar.
- Protections: a click on the bar is the mod's, never the world's, and the
  overlay draws only over container screens.

Verified with `cargo test` (43) and `native.py check` on Nightly `6560641`.

## 0.1.0 — JEI Core

The first playable viewer, on Horizon's native hook points (version 2). No
Nightly is pinned: it builds on any Horizon whose hook points are version 2 or
later.

- The item panel beside container screens: every item the game knows, paged by
  the game, with a search box.
- JEI's search grammar: words, `"quoted phrases"`, `-exclusions`, `|`
  alternatives and the `@namespace` / `#tag` prefixes (`$tooltip` and
  `%creative tab` wait for their data).
- R shows an item's recipes, U its uses, over the panel or any slot of the
  container screen. A click on a panel item does the same.
- Recipe pages with one tab per station (crafting, furnace, blast furnace,
  smoker, campfire, stonecutter, smithing, brewing), the game's five layouts,
  cooking time and XP, and a Back button that walks the history.
- A recipe slot opens that stack's recipes.
- "+" places a recipe the recipe book holds into the open grid, greyed until
  the book has it (no shift yet: the panel's action event carries no
  modifiers).
- "Click gives items" (Options > Mods > JEI, or `/jei give`): in creative a
  click gives one, a right click a stack; in survival the same click opens the
  recipes.
- English and Spanish UI (`Language` on the mod's page). Item names always come
  from the game in the player's language.
- `/jei` reports the table sizes.

Verified on Nightly `1da354b` with `cargo test` (31), `native.py check` and an
offscreen `proof.py` run: row and settings, search, tabs, transfer, history and
an empty search.