
use super::*;

#[test]
fn fps_counter() {
    let mut fps = FpsData::new(3);
    fps.add_fps(1);
    assert_eq!(fps.calculate_average(), 1);
    fps.add_fps(1);
    assert_eq!(fps.calculate_average(), 1);
    fps.add_fps(1);
    assert_eq!(fps.calculate_average(), 1);
    fps.add_fps(4);
    assert_eq!(fps.calculate_average(), 2);
}