use raylib::prelude::*;

struct Ball {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    radius: f32,
}
impl Ball {
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius, Color::WHITE);
    }
}

struct Paddle {
    x: f32,
    y: f32,
    velocity: f32,
    width: f32,
    height: f32,
}
impl Paddle {
    fn get_rect(&mut self) -> Rectangle {
        rrect(
            self.x - self.width / 2.,
            self.y - self.height / 2.,
            self.width,
            self.height,
        )
    }
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.get_rect(), Color::WHITE);
    }
}

fn main() {
    //initialize window and raylib
    let (mut rl, thread) = raylib::init()
        .size(1024, 760)
        .resizable()
        .title("UwU")
        .vsync()
        .build();

    //variables
    let mut ball = Ball {
        x: rl.get_screen_width() as f32 / 2.,
        y: rl.get_screen_height() as f32 / 2.,
        velocity_x: 300.,
        velocity_y: 300.,
        radius: 10.,
    };

    let mut left_paddle = Paddle {
        x: 50.,
        y: (rl.get_screen_height() / 2) as f32,
        velocity: 500.,
        width: 10.,
        height: 100.,
    };

    let mut right_paddle = Paddle {
        x: (rl.get_screen_width() - 50) as f32,
        y: (rl.get_screen_height() / 2) as f32,
        velocity: 500.,
        width: 10.,
        height: 100.,
    };

    'main_loop: while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);

        // move paddles
        if d.is_key_down(KeyboardKey::KEY_W) {
            left_paddle.y -= left_paddle.velocity * d.get_frame_time();
        }

        if d.is_key_down(KeyboardKey::KEY_S) {
            left_paddle.y += left_paddle.velocity * d.get_frame_time();
        }

        if d.is_key_down(KeyboardKey::KEY_UP) {
            right_paddle.y -= left_paddle.velocity * d.get_frame_time();
        }

        if d.is_key_down(KeyboardKey::KEY_DOWN) {
            right_paddle.y += left_paddle.velocity * d.get_frame_time();
        }

        if left_paddle.get_rect().check_collision_circle_rec(
            Vector2 {
                x: ball.x,
                y: ball.y,
            },
            ball.radius,
        ) && ball.velocity_x < 0.
        {
            ball.velocity_x *= -1.1;
            ball.velocity_y =
                (ball.y - left_paddle.y) / (left_paddle.height / 2.) * ball.velocity_x;
        }

        if right_paddle.get_rect().check_collision_circle_rec(
            Vector2 {
                x: ball.x,
                y: ball.y,
            },
            ball.radius,
        ) && ball.velocity_x > 0.
        {
            ball.velocity_x *= -1.;
            ball.velocity_y =
                (ball.y - right_paddle.y) / (right_paddle.height / 2.) * -ball.velocity_x;
        }

        ball.draw(&mut d);
        left_paddle.draw(&mut d);
        right_paddle.draw(&mut d);

        ball.x += ball.velocity_x * d.get_frame_time();
        ball.y += ball.velocity_y * d.get_frame_time();

        if ball.y <= 1. {
            ball.velocity_y *= -1.;
            ball.y = 1.;
        }

        if ball.y >= d.get_screen_height() as f32 {
            ball.velocity_y *= -1.;
            ball.y = d.get_screen_height() as f32 - 1.;
        }

        if ball.velocity_x > 0. {
            right_paddle.velocity = ((300. * ball.velocity_x) / 100.) / 2.;
        } else {
            right_paddle.velocity = ((300. * (ball.velocity_x * -1.)) / 100.) / 2.;
        }
        left_paddle.velocity = right_paddle.velocity;

        //winner stuff

        if ball.x >= d.get_screen_width() as f32 {
            d.clear_background(Color::LIME);
            let text = "LEFT WINS!";
            let text_len = measure_text(text, 70);
            d.draw_text(
                text,
                d.get_screen_width() / 2 - text_len / 2,
                d.get_screen_height() / 2,
                70,
                Color::RED,
            );
        }

        if ball.x <= 0. {
            d.clear_background(Color::LIME);
            let text = "RIGHT WINS!";
            let text_len = measure_text(text, 70);
            d.draw_text(
                text,
                d.get_screen_width() / 2 - text_len / 2,
                d.get_screen_height() / 2,
                70,
                Color::RED,
            );
        }
    }
}
