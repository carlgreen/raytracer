use std::ops::Add;
use std::ops::Div;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color { r: self.r / other, g: self.g / other, b: self.b / other }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_add_color() {
        let col1 = Color {r: 0.1, g: 0.2, b: 0.3};
        let col2 = Color {r: 0.4, g: 0.5, b: 0.6};
        // assert_eq!(Color {r: 0.5, g: 0.7, b: 0.9}, col1 + col2);
        let got = col1 + col2;
        assert!(got.r > 0.499 && got.r < 0.501);
        assert!(got.g > 0.699 && got.g < 0.701);
        assert!(got.b > 0.899 && got.b < 0.901);
    }

    #[test]
    fn test_div_color() {
        let col1 = Color {r: 0.1, g: 0.2, b: 0.3};
        let got = col1 / 3.0;
        assert!(got.r > 0.032 && got.r < 0.034);
        assert!(got.g > 0.065 && got.g < 0.067);
        assert!(got.b > 0.099 && got.b < 0.101);
    }
}
