extern crate matrix_display;
use self::matrix_display::*;
use rand::{sample, thread_rng};

#[derive(Clone)]
pub struct Board {
    main_color: [i32; 17],
}

impl Board {
    pub fn new() -> Board {
        let main_color = [
            0, 247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54, 53, 52,
        ];

        Board { main_color }
    }
    fn square(&self, data: [i32; 16]) -> Vec<cell::Cell<String>> {
        data.iter()
            .cloned()
            .map(|i| {
                (
                    2_f64.powi(i),
                    *self.main_color.get(i as usize).unwrap() as u8,
                )
            })
            .map(|(x, col)| match x as u32 {
                1 => (".".to_string(), col),
                _ => (x.to_string(), col),
            })
            .map(|(s, col)| cell::Cell::new(s, 0, col))
            .collect::<Vec<_>>()
    }
    pub fn print<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let matrix = matrix::Matrix::new(4, self.square(data));
        let format = Format::new(7, 3);
        let display = MatrixDisplay::new(format, matrix);
        display.print(out, &style::BordersStyle::Thick);
    }
    pub fn no_print<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let mut grey_scale = self.clone();
        grey_scale.main_color = [
            0, 255, 251, 248, 246, 244, 242, 241, 240, 239, 238, 237, 236, 235, 234, 233, 232,
        ];
        grey_scale.print(data, out);
    }

    pub fn print_won<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let mut paint = self.clone();
        let mut rng = thread_rng();
        let mut fw = sample(&mut rng, 1..256, 17);
        fw[0] = 0;
        paint.main_color[..17].clone_from_slice(&fw[..17]);
        paint.print(data, out);
    }
    pub fn print_lost<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let mut red_scale = self.clone();
        red_scale.main_color = [
            0, 90, 126, 162, 198, 197, 161, 125, 89, 53, 17, 196, 160, 124, 88, 52, 16,
        ];
        red_scale.print(data, out);
    }
}
