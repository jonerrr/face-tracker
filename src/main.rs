use opencv::{core, highgui, imgproc, objdetect, prelude::*, types, videoio, Result};
use std::{cmp::min_by, collections::VecDeque};

// #[derive(Debug)]
// struct Points {
//     x: Vec<i32>,
//     y: Vec<i32>,
//     center: f32,
// }

#[derive(Debug)]
struct Points {
    faces: VecDeque<core::Point_<i32>>,
    centers: (i32, i32),
}

impl Points {
    fn new(centers: (i32, i32)) -> Self {
        Points {
            faces: VecDeque::from([
                core::Point_ {
                    x: (centers.0 + centers.1 / 2),
                    y: 300,
                },
                // core::Point_ {
                //     x: (centers.0 + centers.1 / 2),
                //     y: 300,
                // },
                // core::Point_ {
                //     x: (centers.0 + centers.1 / 2),
                //     y: 300,
                // },
                // core::Point_ {
                //     x: (centers.0 + centers.1 / 2),
                //     y: 300,
                // },
                // core::Point_ {
                //     x: (centers.0 + centers.1 / 2),
                //     y: 300,
                // },
            ]),
            centers,
        }
    }

    fn get_point(
        &mut self,
        faces: core::Vector<core::Rect_<i32>>,
    ) -> (core::Point_<i32>, core::Point_<i32>) {
        faces.iter().for_each(|f| {
            self.faces.push_back(core::Point_ {
                x: f.x + f.width / 2,
                y: f.y + f.height / 2,
            })
        });

        if self.faces.len() > 5 {
            self.faces.pop_front();
        }

        (*self.faces.back().unwrap(), *self.faces.back().unwrap())
    }
}

// impl Points {
//     fn new(center: f32) -> Self {
//         Points {
//             x: vec![400],
//             y: vec![300],
//             center,
//             // last_face: None,
//         }
//     }

//     fn put_point(&mut self, x: f32, y: f32) {
//         let dx = x - 700.0;
//         let dy = y - 300.0;

//         let px = 50.0 * (dy.atan2(dx)).cos();
//         let py = 50.0 * (dy.atan2(dx)).sin();

//         println!("x: {}\ny: {}", px, py);

//         self.x.push((px + &self.center) as i32);
//         self.y.push(py as i32 + 300);
//         if self.x.len() > 2 {
//             self.x.remove(0);
//             self.y.remove(0);
//         }
//     }

//     fn get_point(&self) -> core::Point_<i32> {
//         core::Point_ {
//             x: &self.x.iter().sum() / *&self.x.len() as i32,
//             y: &self.y.iter().sum() / *&self.y.len() as i32,
//         }
//     }
// }

fn main() -> Result<()> {
    let window = "Stalker";
    highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Error opening default camera");
    }
    let mut face = objdetect::CascadeClassifier::new("/usr/local/Cellar/opencv/4.6.0/share/opencv4/haarcascades/haarcascade_frontalface_default.xml")?;
    // let mut side = objdetect::CascadeClassifier::new(
    //     "/usr/local/Cellar/opencv/4.6.0/share/opencv4/haarcascades/haarcascade_profileface.xml",
    // )?;

    // let mut left_eye = Points::new(800.0);
    // let mut right_eye = Points::new(400.0);
    let mut f = Points::new((400, 800));

    let mut img = Mat::default();

    loop {
        cam.read(&mut img)?;
        let mut gray = Mat::default();
        imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut faces = types::VectorOfRect::new();
        face.detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            10,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size::new(10, 10),
            core::Size::new(0, 0),
        )?;
        // side.detect_multi_scale(
        //     &gray,
        //     &mut faces,
        //     1.1,
        //     10,
        //     objdetect::CASCADE_SCALE_IMAGE,
        //     core::Size::new(10, 10),
        //     core::Size::new(0, 0),
        // )?;

        // imgproc::rectangle(
        //     &mut img,
        //     core::Rect {
        //         x: 0,
        //         y: 0,
        //         width: 5000,
        //         height: 5000,
        //     },
        //     core::Scalar::new(255f64, 255f64, 255f64, 255f64),
        //     -1,
        //     8,
        //     0,
        // )?;

        // Eyebrows
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 400, y: 270 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            180.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            5,
            1,
            0,
        )?;
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 800, y: 270 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            180.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            5,
            1,
            0,
        )?;

        // Iris
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 800, y: 300 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            0.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            -1,
            1,
            0,
        )?;
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 400, y: 300 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            0.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            -1,
            imgproc::FILLED,
            0,
        )?;

        // let target = faces.get(0);

        // let face = faces.iter().map(|p| {
        //     closest_face = min_by(
        //         closest_face,
        //         core::Point_ {
        //             x: *left_eye.x.last().unwrap(),
        //             y: *left_eye.y.last().unwrap(),
        //         },
        //         |a, b| {

        //         },
        //     );
        // });
        // match target {
        //     Ok(t) => {
        //         let mut cx = t.x + t.width / 2;
        //         let mut cy = t.y + t.height / 2;

        //         if faces.len() > 1 {
        //             for f in faces.iter() {
        //                 let dx = (*left_eye.x.last().unwrap() as i32 - (f.x + f.width / 2)).abs();
        //                 let dy = (*left_eye.y.last().unwrap() as i32 - (f.y + f.height / 2)).abs();
        //                 if dx < (cx - (f.x + f.width / 2)).abs()
        //                     || dx < (cy - (f.y + f.height / 2)).abs()
        //                 {
        //                     cx = dx;
        //                     cy = dy;
        //                 }
        //             }
        //         }

        //         left_eye.put_point(cx as f32, cy as f32);
        //         right_eye.put_point(cx as f32, cy as f32);
        //     }
        //     Err(_) => {
        //         left_eye.put_point(400.0, 300.0);
        //         right_eye.put_point(900.0, 300.0);
        //     }
        // }

        f.get_point(faces);

        imgproc::circle(
            &mut img,
            *f.faces.back().unwrap(),
            30,
            core::Scalar::new(200f64, 50f64, 0f64, 0f64),
            -1,
            imgproc::FILLED,
            0,
        )?;
        imgproc::circle(
            &mut img,
            *f.faces.back().unwrap(),
            30,
            core::Scalar::new(200f64, 50f64, 0f64, 0f64),
            -1,
            8,
            0,
        )?;

        let mut flipped = img.clone();
        core::flip(&img, &mut flipped, 1)?;
        // imgproc::put_text(
        //     &mut flipped,
        //     &format!("faces"),
        //     core::Point_ { x: 10, y: 80 },
        //     0,
        //     3.0,
        //     core::Scalar::new(0f64, 0f64, 0f64, 0f64),
        //     3,
        //     3,
        //     false,
        // )?;

        highgui::imshow(window, &flipped)?;

        if highgui::wait_key(5)? > 0 {
            break;
        }
    }
    Ok(())
}
