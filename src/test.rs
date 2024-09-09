

#[test]
fn test_colorful() {
    use colorful::{Color, Colorful};
    for color in Color::iterator() {
        println!("{}", format!("{:?}", color).color(*color));
    }
}