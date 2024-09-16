use ndarray::prelude::*;

use crate::color::{Color, Pallete};

fn vectorise_color(color: Color) -> Array1<f64> {
    match color {
        Pallete::WHITE => array![1.0, 0.0, 0.0, 0.0],
        Pallete::BLACK => array![0.0, 1.0, 0.0, 0.0],
        Pallete::RED => array![0.0, 0.0, 1.0, 0.0],
        Pallete::BLUE => array![0.0, 0.0, 0.0, 1.0],
    }
}

fn colorise_vector(vector: Array1<f64>) -> Color {
    match vector {
        array![1.0, 0.0, 0.0, 0.0] => Pallete::WHITE,
        array![0.0, 1.0, 0.0, 0.0] => Pallete::BLACK,
        array![0.0, 0.0, 1.0, 0.0] => Pallete::RED,
        array![0.0, 0.0, 0.0, 1.0] => Pallete::BLUE,
        _ => panic!("Invalid color vector"),
    }
}

fn calculate_next_color(
    previous_colors: Vec<Array1<f64>>,
    above_colors: Vec<Array1<f64>>,
) -> Array1<f64> {
    let previous_colour_weight_inv = previous_colors
        .iter()
        .fold(Array1::<f64>::zeros(4), |acc, x| acc + x);
    let above_colour_weight_inv = above_colors.iter().fold(Array1::zeros(4), |acc, x| acc + x);
    let previous_color_weight = 1.0 / previous_colour_weight_inv;
    let above_color_weight = 1.0 / above_colour_weight_inv;
    let default_weights = array![0.25, 0.25, 0.25, 0.25];
    let color_weights = previous_color_weight + above_color_weight + default_weights;
    return color_weights;
}

mod test {
    use super::calculate_next_color;

    #[test]
    fn test() {
        let previous_colors = vec![array![1.0, 0.0, 0.0, 0.0], array![0.0, 1.0, 0.0, 0.0]];
        let above_colors = vec![array![0.0, 0.0, 1.0, 0.0], array![0.0, 0.0, 0.0, 1.0]];
        let next_color = calculate_next_color(previous_colors, above_colors);
    }
}
