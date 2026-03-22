use xiangxue::Dimension;

/// Resolve a `Dimension` to an absolute pixel value.
/// `reference` is the parent size along the same axis (for percent values).
pub fn resolve_dimension(dim: &Dimension, reference: f32) -> f32 {
    match dim {
        Dimension::Px(v) => *v,
        Dimension::Percent(pct) => reference * pct / 100.0,
    }
}

/// Resolve a `Dimension` to a 0-1 fraction of the reference size.
/// Used for Cocos PreSize / PrePosition.
pub fn dimension_to_fraction(dim: &Dimension, reference: f32) -> f32 {
    match dim {
        Dimension::Px(v) => {
            if reference != 0.0 {
                v / reference
            } else {
                0.0
            }
        }
        Dimension::Percent(pct) => pct / 100.0,
    }
}

/// Convert CSS `top` value to Cocos Y coordinate (Y-axis flip).
/// In CSS, Y grows downward; in Cocos, Y grows upward.
pub fn css_top_to_cocos_y(css_top: f32, parent_height: f32, node_height: f32) -> f32 {
    parent_height - css_top - node_height
}

/// Flip a CSS-space anchor Y value to Cocos space.
/// CSS: 0=top, 1=bottom → Cocos: 0=bottom, 1=top
pub fn flip_anchor_y(css_y: f32) -> f32 {
    1.0 - css_y
}

/// Format a float to 4 decimal places.
pub fn fmt4(v: f32) -> String {
    format!("{v:.4}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_top_to_cocos_y_basic() {
        // Node at top=50 in 960-high parent, node height=60
        // Cocos Y = 960 - 50 - 60 = 850
        let y = css_top_to_cocos_y(50.0, 960.0, 60.0);
        assert!((y - 850.0).abs() < 0.001);
    }

    #[test]
    fn css_top_to_cocos_y_at_top() {
        // Node at very top (top=0), height=100, parent=960
        let y = css_top_to_cocos_y(0.0, 960.0, 100.0);
        assert!((y - 860.0).abs() < 0.001);
    }

    #[test]
    fn css_top_to_cocos_y_at_bottom() {
        // Node at bottom (top=900), height=60, parent=960
        let y = css_top_to_cocos_y(900.0, 960.0, 60.0);
        assert!((y - 0.0).abs() < 0.001);
    }

    #[test]
    fn resolve_dimension_px() {
        let d = Dimension::Px(200.0);
        assert!((resolve_dimension(&d, 640.0) - 200.0).abs() < 0.001);
    }

    #[test]
    fn resolve_dimension_percent() {
        let d = Dimension::Percent(50.0);
        assert!((resolve_dimension(&d, 640.0) - 320.0).abs() < 0.001);
    }

    #[test]
    fn dimension_to_fraction_px() {
        let d = Dimension::Px(320.0);
        assert!((dimension_to_fraction(&d, 640.0) - 0.5).abs() < 0.001);
    }

    #[test]
    fn dimension_to_fraction_percent() {
        let d = Dimension::Percent(25.0);
        assert!((dimension_to_fraction(&d, 640.0) - 0.25).abs() < 0.001);
    }

    #[test]
    fn dimension_to_fraction_zero_reference() {
        let d = Dimension::Px(100.0);
        assert!((dimension_to_fraction(&d, 0.0) - 0.0).abs() < 0.001);
    }

    #[test]
    fn flip_anchor_y_values() {
        assert!((flip_anchor_y(0.0) - 1.0).abs() < 0.001);
        assert!((flip_anchor_y(1.0) - 0.0).abs() < 0.001);
        assert!((flip_anchor_y(0.5) - 0.5).abs() < 0.001);
        assert!((flip_anchor_y(0.25) - 0.75).abs() < 0.001);
    }

    #[test]
    fn fmt4_formatting() {
        assert_eq!(fmt4(0.0), "0.0000");
        assert_eq!(fmt4(1.5), "1.5000");
        assert_eq!(fmt4(123.4567), "123.4567");
    }
}
