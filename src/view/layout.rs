pub struct Layout {}

impl Layout {
    pub fn render_together(mut list_of_canvases: Vec<Vec<Vec<char>>>) {
        if list_of_canvases.len() == 0 {
            return;
        }

        list_of_canvases.sort_by_key(|canvas| std::cmp::Reverse(canvas.len()));

        let max_size = list_of_canvases[0].len();

        for iter in 0..max_size {
            let mut st = String::new();

            for canv_canv in list_of_canvases.iter() {
                if canv_canv.len() + iter >= max_size {
                    st = st
                        + &canv_canv[iter - (max_size - canv_canv.len())]
                            .iter()
                            .collect::<String>()
                        + "     ";
                }
            }

            println!("{}", st);
        }
    }
}
