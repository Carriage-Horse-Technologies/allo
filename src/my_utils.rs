use web_sys::DomRect;

pub(crate) fn px_to_tws(px: u32) -> u32 {
    px / 4
}

fn check_collision(
    (a_top, a_bottom, a_left, a_right): (f64, f64, f64, f64),
    (b_top, b_bottom, b_left, b_right): (f64, f64, f64, f64),
) -> bool {
    return a_top < b_bottom && a_bottom > b_top && a_left < b_right && a_right > b_left;
}

pub(crate) fn check_collision_with_dom_rect(dom_a: &DomRect, dom_b: &DomRect) -> bool {
    check_collision(
        (dom_a.top(), dom_a.bottom(), dom_a.left(), dom_a.right()),
        (dom_b.top(), dom_b.bottom(), dom_b.left(), dom_b.right()),
    )
}

#[cfg(test)]
mod tests {
    use super::check_collision;

    #[test]
    fn test_check_collision() {
        let obj_a = (0., 100., 50., 150.);
        let obj_b = (50., 150., 25., 200.);
        let obj_c = (50., 150., 200., 250.);

        assert_eq!(check_collision(obj_a, obj_b), true);
        assert_eq!(check_collision(obj_a, obj_c), false);
    }
}
