use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use uuid::Uuid;

use crate::util::{css_top_to_cocos_y, flip_anchor_y, fmt4, resolve_dimension};
use xiangxue::{Backend, UiNode, WidgetKind};

/// Cocos Studio .csd XML backend.
pub struct CocosBackend {
    pub design_width: f32,
    pub design_height: f32,
}

impl Default for CocosBackend {
    fn default() -> Self {
        Self {
            design_width: 640.0,
            design_height: 960.0,
        }
    }
}

impl Backend for CocosBackend {
    type Error = std::fmt::Error;

    fn extension(&self) -> &str {
        "csd"
    }

    fn design_size(&self) -> (f32, f32) {
        (self.design_width, self.design_height)
    }

    fn emit(&self, root: &UiNode) -> Result<Vec<u8>, Self::Error> {
        Ok(self.emit_document(root).into_bytes())
    }
}

impl CocosBackend {
    pub fn emit_document(&self, root: &UiNode) -> String {
        let mut w = XmlWriter::new();
        let project_id = Uuid::new_v4().to_string();

        w.raw("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        w.open_tag("GameProjectFile", &[]);
        w.self_closing_tag(
            "PropertyGroup",
            &[
                ("Type", "Node"),
                ("Name", &root.name),
                ("ID", &project_id),
                ("Version", "2.1.0.0"),
            ],
        );
        w.open_tag("Content", &[("ctype", "GameProjectContent")]);
        w.open_tag("Content", &[]);
        w.self_closing_tag("Animation", &[("Duration", "0"), ("Speed", "1.0000")]);

        let mut tag_counter = 1u32;
        self.emit_node(root, &mut w, &mut tag_counter, true, self.design_width, self.design_height);

        w.close_tag("Content");
        w.close_tag("Content");
        w.close_tag("GameProjectFile");

        w.finish()
    }

    fn emit_node(
        &self,
        node: &UiNode,
        w: &mut XmlWriter,
        tag_counter: &mut u32,
        is_root: bool,
        parent_w: f32,
        parent_h: f32,
    ) {
        let ctype = ctype_for(&node.widget, &node.attrs);
        let action_tag = action_tag_from_name(&node.name).to_string();
        let tag_str = tag_counter.to_string();
        *tag_counter += 1;

        let touch_enable = is_interactive(&node.widget);

        let mut attrs: Vec<(&str, &str)> = vec![
            ("Name", &node.name),
            ("ActionTag", &action_tag),
            ("Tag", &tag_str),
            ("ctype", ctype),
        ];

        // Widget-specific attributes
        let text_val;
        let placeholder_val;
        let gap_str;
        let progress_str;
        match &node.widget {
            WidgetKind::Button => {
                text_val = node.attrs.get("text").cloned().unwrap_or_default();
                attrs.push(("ButtonText", &text_val));
                attrs.push(("FontSize", "14"));
            }
            WidgetKind::Text => {
                text_val = node.attrs.get("text").cloned().unwrap_or_default();
                attrs.push(("LabelText", &text_val));
                attrs.push(("FontSize", "20"));
            }
            WidgetKind::TextField => {
                placeholder_val =
                    node.attrs.get("placeholder").cloned().unwrap_or_default();
                text_val = node.attrs.get("text").cloned().unwrap_or_default();
                attrs.push(("PlaceHolderText", &placeholder_val));
                attrs.push(("LabelText", &text_val));
                attrs.push(("FontSize", "20"));
            }
            WidgetKind::ScrollView => {
                attrs.push(("ScrollDirectionType", "Vertical"));
            }
            WidgetKind::ListView => {
                attrs.push(("DirectionType", "Vertical"));
                gap_str = node.layout.gap.map(|g| g.round() as i32).unwrap_or(0).to_string();
                attrs.push(("ItemMargin", &gap_str));
            }
            WidgetKind::Slider => {
                attrs.push(("PercentInfo", "50"));
            }
            WidgetKind::ProgressBar => {
                progress_str = compute_progress_percent(node);
                attrs.push(("ProgressInfo", &progress_str));
            }
            WidgetKind::Unknown(name) => match name.as_str() {
                "TextBMFont" => {
                    text_val = node.attrs.get("text").cloned().unwrap_or_default();
                    attrs.push(("LabelText", &text_val));
                }
                "TextAtlas" => {
                    text_val = node.attrs.get("text").cloned().unwrap_or_default();
                    attrs.push(("LabelText", &text_val));
                }
                _ => {}
            },
            _ => {}
        }

        // TextAtlas-specific attributes (CharWidth, CharHeight)
        let char_width_str;
        let char_height_str;
        let start_char_str;
        if let WidgetKind::Unknown(name) = &node.widget {
            if name == "TextAtlas" {
                if let Some(cw) = node.attrs.get("data-char-width") {
                    char_width_str = cw.clone();
                    attrs.push(("CharWidth", &char_width_str));
                }
                if let Some(ch) = node.attrs.get("data-char-height") {
                    char_height_str = ch.clone();
                    attrs.push(("CharHeight", &char_height_str));
                }
                if let Some(sc) = node.attrs.get("data-start-char") {
                    start_char_str = sc.clone();
                    attrs.push(("StartChar", &start_char_str));
                }
            }
        }

        if touch_enable {
            attrs.push(("TouchEnable", "True"));
        }

        // Visual property attributes
        let rotation_x_str;
        let rotation_y_str;
        if let Some(deg) = node.layout.rotation {
            rotation_x_str = fmt4(deg);
            rotation_y_str = fmt4(deg);
            attrs.push(("RotationSkewX", &rotation_x_str));
            attrs.push(("RotationSkewY", &rotation_y_str));
        }
        if node.layout.visible == Some(false) {
            attrs.push(("VisibleForFrame", "False"));
        }
        let z_order_str;
        if let Some(z) = node.layout.z_order {
            z_order_str = z.to_string();
            attrs.push(("ZOrder", &z_order_str));
        }
        if node.layout.scale_x.map_or(false, |s| s < 0.0) {
            attrs.push(("FlipX", "True"));
        }
        if node.layout.scale_y.map_or(false, |s| s < 0.0) {
            attrs.push(("FlipY", "True"));
        }
        if matches!(&node.widget, WidgetKind::Layout(_)) && node.layout.background_color.is_some()
        {
            attrs.push(("ComboBoxIndex", "1"));
        }

        w.open_tag("ObjectData", &attrs);

        // Base properties
        self.emit_base_properties(node, w, is_root, parent_w, parent_h);

        // Widget-specific child elements
        self.emit_widget_elements(node, w);

        // Children — use this node's resolved size as parent for children
        if !node.children.is_empty() {
            let this_w = node.layout.resolved_width.unwrap_or(
                if is_root { self.design_width } else { 0.0 }
            );
            let this_h = node.layout.resolved_height.unwrap_or(
                if is_root { self.design_height } else { 0.0 }
            );
            w.open_tag("Children", &[]);
            for child in &node.children {
                self.emit_node(child, w, tag_counter, false, this_w, this_h);
            }
            w.close_tag("Children");
        }

        w.close_tag("ObjectData");
    }

    fn emit_base_properties(
        &self,
        node: &UiNode,
        w: &mut XmlWriter,
        is_root: bool,
        parent_w: f32,
        parent_h: f32,
    ) {
        let layout = &node.layout;

        // ── Size (prefer resolved values) ──
        let node_w = if is_root {
            layout.resolved_width.unwrap_or_else(|| {
                layout.width.as_ref().map(|d| resolve_dimension(d, parent_w))
                    .unwrap_or(self.design_width)
            })
        } else {
            layout.resolved_width.unwrap_or_else(|| {
                layout.width.as_ref().map(|d| resolve_dimension(d, parent_w))
                    .unwrap_or(0.0)
            })
        };
        let node_h = if is_root {
            layout.resolved_height.unwrap_or_else(|| {
                layout.height.as_ref().map(|d| resolve_dimension(d, parent_h))
                    .unwrap_or(self.design_height)
            })
        } else {
            layout.resolved_height.unwrap_or_else(|| {
                layout.height.as_ref().map(|d| resolve_dimension(d, parent_h))
                    .unwrap_or(0.0)
            })
        };

        // ── Position (prefer resolved values, then fallback to manual) ──
        let (cocos_x, cocos_y) = if layout.resolved_x.is_some() || layout.resolved_y.is_some() {
            // Taffy resolved: always Y-flip since resolved_y is a real CSS-space coordinate
            let css_x = layout.resolved_x.unwrap_or(0.0);
            let css_y = layout.resolved_y.unwrap_or(0.0);
            (css_x, css_top_to_cocos_y(css_y, parent_h, node_h))
        } else {
            // Fallback: manual resolution (no resolve_layout called)
            let margin_left = layout.margin_left.unwrap_or(0.0);
            let margin_top = layout.margin_top.unwrap_or(0.0);
            let pos_x = layout.left.as_ref()
                .map(|d| resolve_dimension(d, parent_w) + margin_left)
                .unwrap_or(0.0);
            let cocos_y = match layout.top.as_ref() {
                Some(d) => {
                    let css_top = resolve_dimension(d, parent_h) + margin_top;
                    css_top_to_cocos_y(css_top, parent_h, node_h)
                }
                None => 0.0,
            };
            (pos_x, cocos_y)
        };

        let pos_x_str = fmt4(cocos_x);
        let pos_y_str = fmt4(cocos_y);
        w.self_closing_tag("Position", &[("X", &pos_x_str), ("Y", &pos_y_str)]);

        // ── Scale ──
        let sx = layout.scale_x.unwrap_or(1.0).abs();
        let sy = layout.scale_y.unwrap_or(1.0).abs();
        let sx_str = fmt4(sx);
        let sy_str = fmt4(sy);
        w.self_closing_tag("Scale", &[("ScaleX", &sx_str), ("ScaleY", &sy_str)]);

        // ── AnchorPoint ──
        let (anchor_x, anchor_y) = if is_root {
            (
                layout.anchor_x.unwrap_or(0.0),
                layout.anchor_y.map(flip_anchor_y).unwrap_or(0.0),
            )
        } else {
            (
                layout.anchor_x.unwrap_or(0.5),
                layout.anchor_y.map(flip_anchor_y).unwrap_or(0.5),
            )
        };
        let ax_str = fmt4(anchor_x);
        let ay_str = fmt4(anchor_y);
        w.self_closing_tag("AnchorPoint", &[("ScaleX", &ax_str), ("ScaleY", &ay_str)]);

        // ── CColor ──
        let (cr, cg, cb) = layout.color.unwrap_or((255, 255, 255));
        let alpha = layout
            .opacity
            .map(|o| (o.clamp(0.0, 1.0) * 255.0).round() as u8)
            .unwrap_or(255);
        let a_str = alpha.to_string();
        let r_str = cr.to_string();
        let g_str = cg.to_string();
        let b_str = cb.to_string();
        w.self_closing_tag(
            "CColor",
            &[("A", &a_str), ("R", &r_str), ("G", &g_str), ("B", &b_str)],
        );

        // ── Size ──
        let size_x_str = fmt4(node_w);
        let size_y_str = fmt4(node_h);
        w.self_closing_tag("Size", &[("X", &size_x_str), ("Y", &size_y_str)]);

        // ── PrePosition (fraction of parent, in Cocos coordinates) ──
        let pre_pos_x = if parent_w != 0.0 { cocos_x / parent_w } else { 0.0 };
        let pre_pos_y = if parent_h != 0.0 { cocos_y / parent_h } else { 0.0 };
        let ppx_str = fmt4(pre_pos_x);
        let ppy_str = fmt4(pre_pos_y);
        w.self_closing_tag("PrePosition", &[("X", &ppx_str), ("Y", &ppy_str)]);

        // ── PreSize (fraction of parent) ──
        let pre_size_x = if parent_w != 0.0 { node_w / parent_w } else { 0.0 };
        let pre_size_y = if parent_h != 0.0 { node_h / parent_h } else { 0.0 };
        let psx_str = fmt4(pre_size_x);
        let psy_str = fmt4(pre_size_y);
        w.self_closing_tag("PreSize", &[("X", &psx_str), ("Y", &psy_str)]);
    }

    fn emit_widget_elements(&self, node: &UiNode, w: &mut XmlWriter) {
        // Resolve image source: src attribute or background-image CSS
        let bg_img = node.attrs.get("src").or(node.layout.background_image.as_ref());

        match &node.widget {
            WidgetKind::Image => {
                if let Some(src) = bg_img {
                    w.self_closing_tag("FileData", &[("Path", src), ("Type", "Normal")]);
                }
            }
            WidgetKind::Button => {
                // NormalFileData from src or background-image
                if let Some(src) = bg_img {
                    w.self_closing_tag(
                        "NormalFileData",
                        &[("Path", src), ("Type", "Normal")],
                    );
                }
                // PressedFileData from data-pressed
                if let Some(pressed) = node.attrs.get("data-pressed") {
                    w.self_closing_tag(
                        "PressedFileData",
                        &[("Path", pressed), ("Type", "Normal")],
                    );
                }
                // DisabledFileData from data-disabled
                if let Some(disabled) = node.attrs.get("data-disabled") {
                    w.self_closing_tag(
                        "DisabledFileData",
                        &[("Path", disabled), ("Type", "Normal")],
                    );
                }
            }
            WidgetKind::ScrollView | WidgetKind::ListView => {
                // background-image on scrollview/listview
                if let Some(src) = bg_img {
                    w.self_closing_tag("FileData", &[("Path", src), ("Type", "Normal")]);
                }
                w.self_closing_tag(
                    "InnerNodeSize",
                    &[("Width", "0.0000"), ("Height", "0.0000")],
                );
            }
            WidgetKind::Layout(_) => {
                // Panel background from background-image
                if let Some(src) = bg_img {
                    w.self_closing_tag("FileData", &[("Path", src), ("Type", "Normal")]);
                }
                // Panel background color
                if let Some((r, g, b, a)) = node.layout.background_color {
                    let r_s = r.to_string();
                    let g_s = g.to_string();
                    let b_s = b.to_string();
                    let a_s = a.to_string();
                    w.self_closing_tag("SingleColor", &[("R", &r_s), ("G", &g_s), ("B", &b_s)]);
                    w.self_closing_tag("BackColorAlpha", &[("Value", &a_s)]);
                }
            }
            WidgetKind::Unknown(name) => match name.as_str() {
                "Sprite" => {
                    if let Some(src) = bg_img {
                        w.self_closing_tag("FileData", &[("Path", src), ("Type", "Normal")]);
                    }
                }
                "ProjectNode" => {
                    // Prefer data-file for .csd references, fallback to src/background-image
                    let file = node.attrs.get("data-file").or(bg_img);
                    if let Some(path) = file {
                        w.self_closing_tag("FileData", &[("Path", path), ("Type", "Normal")]);
                    }
                }
                "TextBMFont" => {
                    if let Some(fnt) = node.attrs.get("data-fnt-file") {
                        w.self_closing_tag(
                            "LabelBMFontFile_CNB",
                            &[("Path", fnt), ("Type", "Normal")],
                        );
                    }
                }
                "TextAtlas" => {
                    if let Some(atlas) = node.attrs.get("data-atlas-file") {
                        w.self_closing_tag(
                            "LabelAtlasFileImage_CNB",
                            &[("Path", atlas), ("Type", "Normal")],
                        );
                    }
                }
                _ => {
                    if let Some(src) = bg_img {
                        w.self_closing_tag("FileData", &[("Path", src), ("Type", "Normal")]);
                    }
                }
            }
            _ => {
                // Other widgets (CheckBox, Slider, etc.) with background-image
                if let Some(src) = bg_img {
                    w.self_closing_tag("FileData", &[("Path", src), ("Type", "Normal")]);
                }
            }
        }
    }
}

/// Compute progress percentage from data-value and data-max attributes.
fn compute_progress_percent(node: &UiNode) -> String {
    let value: f32 = node.attrs.get("data-value")
        .and_then(|v| v.parse().ok())
        .or_else(|| node.attrs.get("value").and_then(|v| v.parse().ok()))
        .unwrap_or(0.0);
    let max: f32 = node.attrs.get("data-max")
        .and_then(|v| v.parse().ok())
        .unwrap_or(100.0);
    let percent = if max > 0.0 { (value / max * 100.0).round() as i32 } else { 0 };
    percent.clamp(0, 100).to_string()
}

/// Map WidgetKind to Cocos Studio ctype string.
fn ctype_for(widget: &WidgetKind, _attrs: &std::collections::HashMap<String, String>) -> &'static str {
    match widget {
        WidgetKind::Layout(_) => "PanelObjectData",
        WidgetKind::Button => "ButtonObjectData",
        WidgetKind::Text => "TextObjectData",
        WidgetKind::Image => "ImageViewObjectData",
        WidgetKind::ScrollView => "ScrollViewObjectData",
        WidgetKind::ListView => "ListViewObjectData",
        WidgetKind::TextField => "TextFieldObjectData",
        WidgetKind::CheckBox => "CheckBoxObjectData",
        WidgetKind::Slider => "SliderObjectData",
        WidgetKind::ProgressBar => "LoadingBarObjectData",
        WidgetKind::Unknown(name) => match name.as_str() {
            "PageView" => "PageViewObjectData",
            "Sprite" => "SpriteObjectData",
            "TabControl" => "TabControlObjectData",
            "ProjectNode" => "ProjectNodeObjectData",
            "Node" => "SingleNodeObjectData",
            "TextBMFont" => "TextBMFontObjectData",
            "TextAtlas" => "TextAtlasObjectData",
            _ => {
                // Check if it looks like a known Cocos ctype suffix
                if name.ends_with("ObjectData") {
                    // Caller passed the ctype directly (unusual but supported)
                    // Fall through to SingleNodeObjectData since we can't return a dynamic &str
                    "SingleNodeObjectData"
                } else {
                    "SingleNodeObjectData"
                }
            }
        },
    }
}

/// Returns true for interactive widgets that need TouchEnable="True".
fn is_interactive(widget: &WidgetKind) -> bool {
    matches!(
        widget,
        WidgetKind::Button
            | WidgetKind::TextField
            | WidgetKind::CheckBox
            | WidgetKind::Slider
    )
}

/// Deterministic ActionTag from node name using hash.
fn action_tag_from_name(name: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let h = hasher.finish();
    // Cocos uses negative ActionTags for editor-generated nodes
    -((h & 0x7FFF_FFFF) as i32)
}

/// Simple XML string builder with indentation.
struct XmlWriter {
    buf: String,
    indent: usize,
}

impl XmlWriter {
    fn new() -> Self {
        Self {
            buf: String::new(),
            indent: 0,
        }
    }

    fn raw(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn open_tag(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.write_indent();
        self.buf.push('<');
        self.buf.push_str(name);
        for (k, v) in attrs {
            self.buf.push(' ');
            self.buf.push_str(k);
            self.buf.push_str("=\"");
            self.buf.push_str(&xml_escape(v));
            self.buf.push('"');
        }
        self.buf.push_str(">\n");
        self.indent += 1;
    }

    fn self_closing_tag(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.write_indent();
        self.buf.push('<');
        self.buf.push_str(name);
        for (k, v) in attrs {
            self.buf.push(' ');
            self.buf.push_str(k);
            self.buf.push_str("=\"");
            self.buf.push_str(&xml_escape(v));
            self.buf.push('"');
        }
        self.buf.push_str("/>\n");
    }

    fn close_tag(&mut self, name: &str) {
        self.indent = self.indent.saturating_sub(1);
        self.write_indent();
        self.buf.push_str("</");
        self.buf.push_str(name);
        self.buf.push_str(">\n");
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push_str("  ");
        }
    }

    fn finish(self) -> String {
        self.buf
    }
}

/// Escape XML special characters in attribute values.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use xiangxue::{parse_html, Orientation};
    use std::collections::HashMap;

    fn make_node(name: &str, widget: WidgetKind) -> UiNode {
        UiNode {
            name: name.to_string(),
            widget,
            children: Vec::new(),
            attrs: HashMap::new(),
            css: xiangxue::CssProperties::default(),
            layout: xiangxue::LayoutProps::default(),
        }
    }

    #[test]
    fn single_button_ctype() {
        let mut node = make_node("ok", WidgetKind::Button);
        node.attrs.insert("text".to_string(), "OK".to_string());
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"ButtonObjectData\""));
        assert!(xml.contains("ButtonText=\"OK\""));
    }

    #[test]
    fn nested_layout_children() {
        let child = make_node("inner", WidgetKind::Text);
        let mut root = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        root.children.push(child);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&root);
        assert!(xml.contains("<Children>"));
        assert!(xml.contains("ctype=\"PanelObjectData\""));
        assert!(xml.contains("ctype=\"TextObjectData\""));
    }

    #[test]
    fn image_with_src_filedata() {
        let mut node = make_node("icon", WidgetKind::Image);
        node.attrs.insert("src".to_string(), "a.png".to_string());
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"ImageViewObjectData\""));
        assert!(xml.contains("Path=\"a.png\""));
        assert!(xml.contains("Type=\"Normal\""));
    }

    #[test]
    fn checkbox_ctype() {
        let node = make_node("toggle", WidgetKind::CheckBox);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"CheckBoxObjectData\""));
        assert!(xml.contains("TouchEnable=\"True\""));
    }

    #[test]
    fn unknown_pageview_ctype() {
        let node = make_node("pages", WidgetKind::Unknown("PageView".to_string()));
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"PageViewObjectData\""));
    }

    #[test]
    fn slider_ctype() {
        let node = make_node("vol", WidgetKind::Slider);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"SliderObjectData\""));
        assert!(xml.contains("PercentInfo=\"50\""));
    }

    #[test]
    fn progress_bar_ctype() {
        let node = make_node("hp", WidgetKind::ProgressBar);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"LoadingBarObjectData\""));
        // No data-value/data-max attrs → defaults to 0%
        assert!(xml.contains("ProgressInfo=\"0\""));
    }

    #[test]
    fn progress_bar_with_data_attrs() {
        let mut node = make_node("hp", WidgetKind::ProgressBar);
        node.attrs.insert("data-value".to_string(), "850".to_string());
        node.attrs.insert("data-max".to_string(), "1060".to_string());
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"LoadingBarObjectData\""));
        // 850/1060*100 ≈ 80
        assert!(xml.contains("ProgressInfo=\"80\""));
    }

    #[test]
    fn full_document_structure() {
        let backend = CocosBackend::default();
        let node = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        let xml = backend.emit_document(&node);
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
        assert!(xml.contains("<GameProjectFile>"));
        assert!(xml.contains("</GameProjectFile>"));
        assert!(xml.contains("<PropertyGroup"));
        assert!(xml.contains("ctype=\"GameProjectContent\""));
        assert!(xml.contains("<Animation"));
    }

    #[test]
    fn root_size_uses_design_dimensions() {
        let backend = CocosBackend {
            design_width: 1280.0,
            design_height: 720.0,
        };
        let node = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        let xml = backend.emit_document(&node);
        assert!(xml.contains("X=\"1280.0000\""));
        assert!(xml.contains("Y=\"720.0000\""));
    }

    #[test]
    fn root_anchor_is_zero() {
        let backend = CocosBackend::default();
        let mut root = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        root.children
            .push(make_node("child", WidgetKind::Button));
        let xml = backend.emit_document(&root);
        // Root should have AnchorPoint 0,0 - appears first
        // Child should have AnchorPoint 0.5,0.5
        let first_anchor = xml.find("AnchorPoint").unwrap();
        let anchor_segment = &xml[first_anchor..first_anchor + 60];
        assert!(anchor_segment.contains("0.0000"));
    }

    #[test]
    fn html_to_csd_integration() {
        let html = r#"<div data-name="root"><div data-widget="Button" id="ok">OK</div><img data-name="icon" src="a.png"/></div>"#;
        let tree = parse_html(html);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&tree);
        assert!(xml.contains("ctype=\"PanelObjectData\""));
        assert!(xml.contains("ctype=\"ButtonObjectData\""));
        assert!(xml.contains("ButtonText=\"OK\""));
        assert!(xml.contains("ctype=\"ImageViewObjectData\""));
        assert!(xml.contains("Path=\"a.png\""));
    }

    #[test]
    fn html_checkbox_to_csd() {
        let html = r#"<div data-name="root"><div data-widget="CheckBox" data-name="toggle"></div></div>"#;
        let tree = parse_html(html);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&tree);
        assert!(xml.contains("ctype=\"CheckBoxObjectData\""));
    }

    #[test]
    fn action_tag_deterministic() {
        let a = action_tag_from_name("button1");
        let b = action_tag_from_name("button1");
        assert_eq!(a, b);
        assert!(a < 0); // ActionTags are negative
    }

    #[test]
    fn xml_escape_special_chars() {
        let escaped = xml_escape("a&b<c>d\"e'f");
        assert_eq!(escaped, "a&amp;b&lt;c&gt;d&quot;e&apos;f");
    }

    #[test]
    fn textfield_attributes() {
        let mut node = make_node("email", WidgetKind::TextField);
        node.attrs
            .insert("placeholder".to_string(), "Enter email".to_string());
        node.attrs
            .insert("text".to_string(), "hello".to_string());
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("PlaceHolderText=\"Enter email\""));
        assert!(xml.contains("LabelText=\"hello\""));
        assert!(xml.contains("ctype=\"TextFieldObjectData\""));
    }

    #[test]
    fn scrollview_inner_node_size() {
        let node = make_node("scroll", WidgetKind::ScrollView);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"ScrollViewObjectData\""));
        assert!(xml.contains("<InnerNodeSize"));
    }

    #[test]
    fn listview_attributes() {
        let node = make_node("list", WidgetKind::ListView);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"ListViewObjectData\""));
        assert!(xml.contains("DirectionType=\"Vertical\""));
        assert!(xml.contains("ItemMargin=\"0\""));
        assert!(xml.contains("<InnerNodeSize"));
    }

    #[test]
    fn data_widget_to_csd() {
        let html = r#"<div data-widget="PageView" data-name="pages"></div>"#;
        let tree = parse_html(html);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&tree);
        assert!(xml.contains("ctype=\"PageViewObjectData\""));
    }

    // ── New layout-aware tests ──────────────────────────────────────────

    #[test]
    fn child_with_size() {
        let mut child = make_node("btn", WidgetKind::Button);
        child.layout.width = Some(xiangxue::Dimension::Px(200.0));
        child.layout.height = Some(xiangxue::Dimension::Px(60.0));
        let mut root = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        root.children.push(child);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&root);
        // Child Size should be 200x60
        assert!(xml.contains("<Size X=\"200.0000\" Y=\"60.0000\"/>"));
    }

    #[test]
    fn percent_size_to_presize() {
        let mut child = make_node("panel", WidgetKind::Layout(Orientation::Vertical));
        child.layout.width = Some(xiangxue::Dimension::Percent(50.0));
        child.layout.height = Some(xiangxue::Dimension::Percent(25.0));
        let mut root = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        root.children.push(child);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&root);
        // PreSize should be (0.5, 0.25)
        assert!(xml.contains("<PreSize X=\"0.5000\" Y=\"0.2500\"/>"));
        // Size should be resolved: 640*0.5=320, 960*0.25=240
        assert!(xml.contains("<Size X=\"320.0000\" Y=\"240.0000\"/>"));
    }

    #[test]
    fn child_with_position_y_flipped() {
        let mut child = make_node("btn", WidgetKind::Button);
        child.layout.width = Some(xiangxue::Dimension::Px(200.0));
        child.layout.height = Some(xiangxue::Dimension::Px(60.0));
        child.layout.left = Some(xiangxue::Dimension::Px(100.0));
        child.layout.top = Some(xiangxue::Dimension::Px(50.0));
        let mut root = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        root.children.push(child);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&root);
        // Position X = 100, Position Y = 960 - 50 - 60 = 850
        assert!(xml.contains("<Position X=\"100.0000\" Y=\"850.0000\"/>"));
    }

    #[test]
    fn custom_anchor_flipped() {
        let mut child = make_node("btn", WidgetKind::Button);
        // CSS anchor (0,0) = left-top → Cocos (0, 1.0)
        child.layout.anchor_x = Some(0.0);
        child.layout.anchor_y = Some(0.0);
        let mut root = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        root.children.push(child);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&root);
        // AnchorPoint for child: (0.0, flip(0.0)) = (0.0, 1.0)
        assert!(xml.contains("<AnchorPoint ScaleX=\"0.0000\" ScaleY=\"1.0000\"/>"));
    }

    #[test]
    fn default_layout_matches_level0() {
        // A node with LayoutProps::default() should produce the same output as before
        let child = make_node("child", WidgetKind::Button);
        let mut root = make_node("root", WidgetKind::Layout(Orientation::Vertical));
        root.children.push(child);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&root);
        // Root: Position (0,0), AnchorPoint (0,0), Size (640,960)
        // Child: Position (0,0), AnchorPoint (0.5,0.5), Size (0,0)
        let lines: Vec<&str> = xml.lines().collect();
        // Find child's properties (after the root's)
        let child_section = xml.find("Name=\"child\"").unwrap();
        let child_xml = &xml[child_section..];
        assert!(child_xml.contains("Position X=\"0.0000\" Y=\"0.0000\""));
        assert!(child_xml.contains("AnchorPoint ScaleX=\"0.5000\" ScaleY=\"0.5000\""));
        assert!(child_xml.contains("Size X=\"0.0000\" Y=\"0.0000\""));
        assert!(child_xml.contains("PrePosition X=\"0.0000\" Y=\"0.0000\""));
        assert!(child_xml.contains("PreSize X=\"0.0000\" Y=\"0.0000\""));
        // Suppress unused variable warning
        let _ = lines;
    }

    #[test]
    fn listview_gap_to_item_margin() {
        let mut node = make_node("list", WidgetKind::ListView);
        node.layout.gap = Some(10.0);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ItemMargin=\"10\""));
    }

    // ── Phase 1: Visual property tests ──────────────────────────────────

    #[test]
    fn scale_output() {
        let mut node = make_node("btn", WidgetKind::Button);
        node.layout.scale_x = Some(2.0);
        node.layout.scale_y = Some(1.5);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ScaleX=\"2.0000\""));
        assert!(xml.contains("ScaleY=\"1.5000\""));
    }

    #[test]
    fn default_scale_unchanged() {
        let node = make_node("btn", WidgetKind::Button);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ScaleX=\"1.0000\""));
        assert!(xml.contains("ScaleY=\"1.0000\""));
    }

    #[test]
    fn negative_scale_flip_output() {
        let mut node = make_node("flipped", WidgetKind::Image);
        node.layout.scale_x = Some(-1.0);
        node.layout.scale_y = Some(1.0);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("FlipX=\"True\""));
        assert!(xml.contains("ScaleX=\"1.0000\""));
        assert!(!xml.contains("FlipY"));
    }

    #[test]
    fn opacity_to_alpha() {
        let mut node = make_node("label", WidgetKind::Text);
        node.layout.opacity = Some(0.5);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("A=\"128\""));
    }

    #[test]
    fn color_output() {
        let mut node = make_node("label", WidgetKind::Text);
        node.layout.color = Some((255, 0, 0));
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("R=\"255\""));
        assert!(xml.contains("G=\"0\""));
        assert!(xml.contains("B=\"0\""));
    }

    #[test]
    fn default_color_is_white() {
        let node = make_node("label", WidgetKind::Text);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("A=\"255\" R=\"255\" G=\"255\" B=\"255\""));
    }

    #[test]
    fn rotation_output() {
        let mut node = make_node("icon", WidgetKind::Image);
        node.layout.rotation = Some(45.0);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("RotationSkewX=\"45.0000\""));
        assert!(xml.contains("RotationSkewY=\"45.0000\""));
    }

    #[test]
    fn visibility_false_output() {
        let mut node = make_node("hidden", WidgetKind::Text);
        node.layout.visible = Some(false);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("VisibleForFrame=\"False\""));
    }

    #[test]
    fn z_order_output() {
        let mut node = make_node("overlay", WidgetKind::Layout(Orientation::Vertical));
        node.layout.z_order = Some(10);
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ZOrder=\"10\""));
    }

    #[test]
    fn background_color_panel() {
        let mut node = make_node("panel", WidgetKind::Layout(Orientation::Vertical));
        node.layout.background_color = Some((100, 150, 200, 180));
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ComboBoxIndex=\"1\""));
        assert!(xml.contains("<SingleColor"));
        assert!(xml.contains("R=\"100\""));
        assert!(xml.contains("<BackColorAlpha"));
        assert!(xml.contains("Value=\"180\""));
    }

    // ── Phase 2: New Cocos type tests ───────────────────────────────────

    #[test]
    fn text_bmfont_ctype_and_fnt() {
        let mut node = make_node("score", WidgetKind::Unknown("TextBMFont".to_string()));
        node.attrs
            .insert("text".to_string(), "12345".to_string());
        node.attrs
            .insert("data-fnt-file".to_string(), "fonts/score.fnt".to_string());
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"TextBMFontObjectData\""));
        assert!(xml.contains("LabelText=\"12345\""));
        assert!(xml.contains("Path=\"fonts/score.fnt\""));
        assert!(xml.contains("LabelBMFontFile_CNB"));
    }

    #[test]
    fn text_atlas_ctype_and_atlas() {
        let mut node = make_node("timer", WidgetKind::Unknown("TextAtlas".to_string()));
        node.attrs
            .insert("text".to_string(), "00:30".to_string());
        node.attrs.insert(
            "data-atlas-file".to_string(),
            "fonts/numbers.png".to_string(),
        );
        node.attrs
            .insert("data-char-width".to_string(), "24".to_string());
        node.attrs
            .insert("data-char-height".to_string(), "32".to_string());
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"TextAtlasObjectData\""));
        assert!(xml.contains("LabelText=\"00:30\""));
        assert!(xml.contains("Path=\"fonts/numbers.png\""));
        assert!(xml.contains("CharWidth=\"24\""));
        assert!(xml.contains("CharHeight=\"32\""));
    }

    #[test]
    fn sprite_ctype_and_filedata() {
        let mut node = make_node("bg", WidgetKind::Unknown("Sprite".to_string()));
        node.attrs
            .insert("src".to_string(), "textures/bg.png".to_string());
        let backend = CocosBackend::default();
        let xml = backend.emit_document(&node);
        assert!(xml.contains("ctype=\"SpriteObjectData\""));
        assert!(xml.contains("Path=\"textures/bg.png\""));
    }
}
