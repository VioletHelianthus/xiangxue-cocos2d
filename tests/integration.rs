use xiangxue::{parse_html, resolve_layout, Dimension, WidgetKind};
use xiangxue_cocos::CocosBackend;

#[test]
fn layout_props_to_csd_integration() {
    let html = r#"<div data-name="root" style="width:640px;height:960px">
  <div data-widget="Button" id="ok" style="width:200px;height:60px;left:220px;top:450px;transform-origin:center">OK</div>
  <img data-name="icon" src="a.png" style="width:100px;height:100px;left:50px;top:50px" data-anchor="0,0"/>
</div>"#;

    let tree = parse_html(html);

    assert_eq!(tree.layout.width, Some(Dimension::Px(640.0)));
    assert_eq!(tree.layout.height, Some(Dimension::Px(960.0)));

    let button = &tree.children[0];
    assert_eq!(button.layout.width, Some(Dimension::Px(200.0)));
    assert_eq!(button.layout.height, Some(Dimension::Px(60.0)));
    assert_eq!(button.layout.anchor_x, Some(0.5));
    assert_eq!(button.layout.anchor_y, Some(0.5));

    let backend = CocosBackend::default();
    let csd = backend.emit_document(&tree);

    assert!(csd.contains("<Size X=\"640.0000\" Y=\"960.0000\"/>"));

    let btn_section = csd.find("Name=\"ok\"").unwrap();
    let btn_xml = &csd[btn_section..];
    assert!(btn_xml.contains("Size X=\"200.0000\" Y=\"60.0000\""));
    assert!(btn_xml.contains("AnchorPoint ScaleX=\"0.5000\" ScaleY=\"0.5000\""));

    let icon_section = csd.find("Name=\"icon\"").unwrap();
    let icon_xml = &csd[icon_section..];
    assert!(icon_xml.contains("AnchorPoint ScaleX=\"0.0000\" ScaleY=\"1.0000\""));
}

#[test]
fn flex_layout_csd_positions() {
    let html = r#"<div data-name="root" style="width:640px;height:960px;display:flex;flex-direction:column">
        <div data-name="header" style="width:640px;height:80px;display:flex;flex-direction:row">
            <div data-name="left" style="width:320px;height:80px"></div>
            <div data-name="right" style="width:320px;height:80px"></div>
        </div>
        <div data-name="body" style="width:640px;height:880px"></div>
    </div>"#;

    let mut tree = parse_html(html);
    resolve_layout(&mut tree, 640.0, 960.0);
    let backend = CocosBackend::default();
    let csd = backend.emit_document(&tree);

    assert!(csd.contains("<Size X=\"640.0000\" Y=\"960.0000\"/>"));

    let header_section = csd.find("Name=\"header\"").unwrap();
    let header_xml = &csd[header_section..];
    assert!(header_xml.contains("Size X=\"640.0000\" Y=\"80.0000\""));
    assert!(header_xml.contains("Position X=\"0.0000\" Y=\"880.0000\""));

    let right_section = csd.find("Name=\"right\"").unwrap();
    let right_xml = &csd[right_section..];
    assert!(right_xml.contains("Position X=\"320.0000\""));
    assert!(right_xml.contains("Size X=\"320.0000\" Y=\"80.0000\""));
}

#[test]
fn flex_grow_csd_output() {
    let html = r#"<div data-name="root" style="width:300px;height:100px;display:flex;flex-direction:row">
        <div data-name="a" style="flex-grow:1;height:50px"></div>
        <div data-name="b" style="flex-grow:2;height:50px"></div>
    </div>"#;

    let mut tree = parse_html(html);
    resolve_layout(&mut tree, 300.0, 100.0);
    let backend = CocosBackend {
        design_width: 300.0,
        design_height: 100.0,
    };
    let csd = backend.emit_document(&tree);

    let a_section = csd.find("Name=\"a\"").unwrap();
    let a_xml = &csd[a_section..];
    assert!(a_xml.contains("Size X=\"100.0000\""));

    let b_section = csd.find("Name=\"b\"").unwrap();
    let b_xml = &csd[b_section..];
    assert!(b_xml.contains("Size X=\"200.0000\""));
}

#[test]
fn progress_bar_csd_uses_data_attributes() {
    let html = r#"<div data-name="root" style="width:640px;height:960px">
        <div data-widget="ProgressBar" data-name="hpBar"
             data-value="850" data-max="1060"
             style="width:200px;height:20px;left:100px;top:100px">
        </div>
    </div>"#;

    let tree = parse_html(html);
    let backend = CocosBackend::default();
    let csd = backend.emit_document(&tree);

    assert!(csd.contains("ctype=\"LoadingBarObjectData\""));

    let hp_section = csd.find("Name=\"hpBar\"").unwrap();
    let hp_xml = &csd[hp_section..];
    assert!(hp_xml.contains("ProgressInfo=\"80\""));
}

#[test]
fn data_widget_textbmfont_integration() {
    let html = r#"<div data-name="root" style="width:640px;height:960px">
        <div data-widget="TextBMFont" data-name="score" data-fnt-file="fonts/score.fnt">12345</div>
    </div>"#;
    let tree = parse_html(html);
    assert_eq!(tree.children[0].widget, WidgetKind::Unknown("TextBMFont".to_string()));
    assert_eq!(tree.children[0].attrs.get("text").map(|s| s.as_str()), Some("12345"));
    assert!(tree.children[0].children.is_empty());

    let backend = CocosBackend::default();
    let csd = backend.emit_document(&tree);
    assert!(csd.contains("ctype=\"TextBMFontObjectData\""));
    assert!(csd.contains("LabelText=\"12345\""));
    assert!(csd.contains("LabelBMFontFile_CNB"));
}

#[test]
fn css_visual_properties_integration() {
    let html = r#"<div data-name="root" style="width:640px;height:960px">
        <span data-name="title" style="color:#ff0000;opacity:0.8">Hello</span>
        <div data-name="panel" style="width:200px;height:100px;background-color:rgba(0,0,255,0.5);visibility:hidden;z-index:5">
        </div>
        <img data-name="icon" src="a.png" style="width:64px;height:64px;transform:scale(2) rotate(45deg)"/>
    </div>"#;

    let tree = parse_html(html);
    let backend = CocosBackend::default();
    let csd = backend.emit_document(&tree);

    let title_section = csd.find("Name=\"title\"").unwrap();
    let title_xml = &csd[title_section..];
    assert!(title_xml.contains("R=\"255\""));
    assert!(title_xml.contains("G=\"0\""));
    assert!(title_xml.contains("A=\"204\""));

    assert!(csd.contains("VisibleForFrame=\"False\""));
    assert!(csd.contains("ZOrder=\"5\""));
    assert!(csd.contains("ComboBoxIndex=\"1\""));
    assert!(csd.contains("<SingleColor"));
    assert!(csd.contains("<BackColorAlpha"));

    let icon_section = csd.find("Name=\"icon\"").unwrap();
    let icon_xml = &csd[icon_section..];
    assert!(icon_xml.contains("ScaleX=\"2.0000\""));
    assert!(icon_xml.contains("ScaleY=\"2.0000\""));
    assert!(icon_xml.contains("RotationSkewX=\"45.0000\""));
}
