---
name: cocos-backend
description: Reference for the xiangxue Cocos2d-x backend crate. Use when working on the Cocos backend code, adding new widget types, fixing CSD generation, modifying coordinate transforms, or understanding how UiNode maps to Cocos Studio .csd XML. Also use when debugging converter output or adding support for new Cocos properties.
---

# Cocos Backend Reference

This crate (`xiangxue-cocos`) implements the `Backend` trait from xiangxue core to generate Cocos Studio `.csd` XML files.

## Crate Structure

```
src/
  lib.rs       — pub mod backend; pub mod util; pub use CocosBackend
  main.rs      — 4 lines: xiangxue::run_cli(|w,h| CocosBackend { ... })
  backend.rs   — CocosBackend implementation (emit_document, emit_node, etc.)
  util.rs      — Coordinate transforms (CSS→Cocos)
```

## Backend Trait Implementation

```rust
impl Backend for CocosBackend {
    type Error = std::fmt::Error;
    fn extension(&self) -> &str { "csd" }
    fn design_size(&self) -> (f32, f32) { (self.design_width, self.design_height) }
    fn emit(&self, root: &UiNode) -> Result<Vec<u8>, Self::Error> { ... }
}
```

## Key Functions in backend.rs

Read `src/backend.rs` for full implementation. Key functions:

- `emit_document(&self, root: &UiNode) -> String` — Top-level: XML declaration + GameProjectFile wrapper
- `emit_node(...)` — Per-node: creates ObjectData with ctype, widget-specific attributes, base properties, children
- `emit_base_properties(...)` — Position, Scale, AnchorPoint, CColor, Size, PrePosition, PreSize
- `emit_widget_elements(...)` — FileData, NormalFileData, InnerNodeSize etc. per widget type
- `ctype_for(widget) -> &str` — WidgetKind → Cocos ctype string mapping
- `is_interactive(widget) -> bool` — Which widgets get TouchEnable="True"

## Coordinate System

CSS space: origin top-left, Y-down.
Cocos space: origin bottom-left, Y-up.

Key transforms in `util.rs`:
- `css_top_to_cocos_y(css_top, parent_h, node_h)` — Flips Y axis
- `flip_anchor_y(css_anchor_y)` — CSS 0=top → Cocos 0=bottom
- `resolve_dimension(dim, reference)` — Dimension enum to px value
- `fmt4(f32)` — Format to 4 decimal places for CSD XML

## Supported Cocos Types

| WidgetKind | ctype | Notes |
|---|---|---|
| Layout(_) | PanelObjectData | Orientation from flex-direction |
| Button | ButtonObjectData | NormalFileData, PressedFileData, DisabledFileData |
| Text | TextObjectData | LabelText, FontSize |
| Image | ImageViewObjectData | FileData from src/background-image |
| ScrollView | ScrollViewObjectData | ScrollDirectionType, InnerNodeSize |
| ListView | ListViewObjectData | DirectionType, ItemMargin from gap |
| TextField | TextFieldObjectData | PlaceHolderText, LabelText |
| CheckBox | CheckBoxObjectData | TouchEnable |
| Slider | SliderObjectData | PercentInfo |
| ProgressBar | LoadingBarObjectData | ProgressInfo from data-value/data-max |
| Unknown("PageView") | PageViewObjectData | |
| Unknown("Sprite") | SpriteObjectData | FileData |
| Unknown("TabControl") | TabControlObjectData | |
| Unknown("ProjectNode") | ProjectNodeObjectData | FileData from data-file or src |
| Unknown("TextBMFont") | TextBMFontObjectData | LabelBMFontFile_CNB |
| Unknown("TextAtlas") | TextAtlasObjectData | LabelAtlasFileImage_CNB, CharWidth/Height |

## Vue Components

The `frontend/src/components/` directory contains 12 Cocos Vue components + ProgressBar.
These are copied into project templates for browser preview and SSR rendering.
Read `references/component-props.md` for the full props reference.

## Testing

```bash
cargo test -p xiangxue-cocos  # 54 tests
```

Unit tests are in `backend.rs` (48 tests). Integration tests in `tests/integration.rs` (6 tests).
