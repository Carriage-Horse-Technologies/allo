use web_sys::DomRect;

pub(crate) fn px_to_tws(px: u32) -> u32 {
    px / 4
}

pub(crate) fn check_collision(
    (a_top, a_bottom, a_left, a_right): (f64, f64, f64, f64),
    (b_top, b_bottom, b_left, b_right): (f64, f64, f64, f64),
) -> bool {
    return a_top < b_bottom && a_bottom > b_top && a_left < b_right && a_right > b_left;
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
