use colored::{ColoredString, Colorize};

pub struct Dial {}

impl Dial {
    const SCALE_MULTIPLIER: f32 = 2.0;
    const HANDS_SYMBOL: char = '█';
    const BORDER_SYMBOL: char = '█';

    pub fn render_dial(percentage: &f32, diameter: &i32) -> Vec<Vec<String>> {
        let canvas_width = diameter.to_owned() * Dial::SCALE_MULTIPLIER as i32;

        let center_x: i32 = canvas_width / 2;
        let radius: i32 = diameter / 2;
        let center_y: i32 = radius;

        let mut canvas = vec![
            vec![String::from(" "); (canvas_width + 1) as usize];
            (radius.to_owned() + 1) as usize
        ];

        Self::render_hand(
            &mut canvas,
            percentage.to_owned(),
            radius,
            center_x,
            center_y,
        );
        Self::render_outer_border(&mut canvas, radius, center_x, center_y);

        canvas
    }

    fn render_outer_border(
        canvas: &mut Vec<Vec<String>>,
        radius: i32,
        center_x: i32,
        center_y: i32,
    ) {
        for ang in 0..180 {
            let border_radians = (ang as f32).to_radians();

            let border_x = center_x
                + (border_radians.cos() * Dial::SCALE_MULTIPLIER * radius as f32).round() as i32;
            let border_y = center_y - (border_radians.sin() * radius as f32).round() as i32;

            canvas[border_y as usize][border_x as usize] = String::from(Self::BORDER_SYMBOL);
        }
    }

    fn render_hand(
        canvas: &mut Vec<Vec<String>>,
        percentage: f32,
        radius: i32,
        center_x: i32,
        center_y: i32,
    ) {
        let angle = 180.00 - (percentage / 100.00 * 180.00);
        let radians = angle.to_radians();

        let dx = radians.cos();
        let dy = radians.sin();

        for y in 0..radius {
            let x_coordinate = center_x + ((dx * Dial::SCALE_MULTIPLIER * y as f32) as i32);
            let y_coordinate = center_y - ((dy * y as f32) as i32);
            canvas[y_coordinate as usize][x_coordinate as usize] =
                Self::get_colored_symbol(&String::from(Self::HANDS_SYMBOL), percentage).to_string();
        }
    }

    fn get_colored_symbol(symbol: &str, percentage: f32) -> ColoredString {
        match percentage {
            p if p > 80.00 => symbol.red(),
            p if p > 50.00 => symbol.yellow(),
            _ => symbol.green(),
        }
    }
}
