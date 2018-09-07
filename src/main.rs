extern crate piston_window;

mod game;

use piston_window::*;

const WHITE : types::Color = [1.0, 1.0, 1.0, 1.0];
const TRANSPARENT : types::Color = [0.0, 0.0, 0.0, 0.0];
const BLACK : types::Color = [0.0, 0.0, 0.0, 1.0];
const UNIT_RECTANGLE: [f64; 4] = [0.0, 0.0, 1.0, 1.0];
const SCREEN_WIDTH : f64 = 640.0;
const SCREEN_HEIGHT : f64 = 480.0;
const BOARD_RADIUS : f64 = 0.003;
const MARKER_RADIUS : f64 = BOARD_RADIUS * 8.0;
const TIC_TAC_TOE_LINE : Line = Line {
    color: BLACK,
    radius: BOARD_RADIUS,
    shape: line::Shape::Square,
};
const TIC_TAC_TOE_LINES : [[f64; 4]; 4] = [
    [1.0 / 3.0, 0.0, 1.0 / 3.0, 1.0],
    [2.0 / 3.0, 0.0, 2.0 / 3.0, 1.0],
    [0.0, 1.0 / 3.0, 1.0, 1.0 / 3.0],
    [0.0, 2.0 / 3.0, 1.0, 2.0 / 3.0],
];
const NOUGHT: Ellipse = Ellipse {
    color: TRANSPARENT,
    border: Option::Some(ellipse::Border {
        color: BLACK,
        radius: MARKER_RADIUS,
    }),
    resolution: 128,
};
const CROSS_LINE : Line = Line {
    color: BLACK,
    radius: MARKER_RADIUS,
    shape: line::Shape::Round,
};
const CROSS_RECTANGLES: [[f64; 4]; 2] = [
    [0.0, 0.0, 1.0, 1.0],
    [0.0, 1.0, 1.0, 0.0],
];

fn draw_nought(draw_state : DrawState, transform : math::Matrix2d, graphics : &mut G2d) {
    NOUGHT.draw(UNIT_RECTANGLE, &draw_state, transform, graphics);
}


fn draw_cross(draw_state : DrawState, transform : math::Matrix2d, graphics : &mut G2d) {
    for line_coordinates in CROSS_RECTANGLES.iter() {
        CROSS_LINE.draw(*line_coordinates, &draw_state, transform, graphics);
    }
}


fn draw_board(draw_state : DrawState, transform : math::Matrix2d, graphics : &mut G2d) {
    for line_coordinates in TIC_TAC_TOE_LINES.iter() {
        TIC_TAC_TOE_LINE.draw(*line_coordinates, &draw_state, transform, graphics);
    }
}


fn draw_pieces(board_pieces : &game::Board, draw_state : DrawState, transform : math::Matrix2d, graphics : &mut G2d) {
    for (x, column) in board_pieces.iter().enumerate() {
        for (y, piece_option) in column.iter().enumerate() {
            if let Some(piece) = piece_option {
                let piece_transform = transform.trans(x as f64, y as f64)
                    .scale(0.6, 0.6)
                    .trans(0.3, 0.3);
                match piece {
                    game::Piece::Cross => draw_cross(draw_state, piece_transform, graphics),
                    game::Piece::Nought => draw_nought(draw_state, piece_transform, graphics),
                }
            }
        }
    }
}


fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Super Tic-Tac-Toe", [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32])
            .opengl(OpenGL::V4_1)
            .exit_on_esc(true).build().unwrap();
    let mut cursor = [0.0, 0.0];
    let mut game_state = game::GameState::new() ;
    while let Some(event) = window.next() {
        window.draw_2d(&event, | context, graphics | {
            clear(WHITE, graphics);
            let window_transform = context.transform.scale(SCREEN_WIDTH, SCREEN_HEIGHT);
            let board_scalar = 1.0 / game::BOARD_LENGTH as f64;
            let board_transform = window_transform.scale(board_scalar, board_scalar);
            for x in 0..game::BOARD_LENGTH {
                for y in 0..game::BOARD_LENGTH {
                    draw_board(context.draw_state, board_transform.trans(x as f64, y as f64), graphics);
                    draw_pieces(
                        &game_state.pieces[x][y], context.draw_state,
                        board_transform.trans(x as f64, y as f64).scale(board_scalar, board_scalar),
                        graphics);
                }
            }
            draw_board(context.draw_state, window_transform, graphics);
            draw_pieces(&game_state.meta_pieces, context.draw_state, board_transform, graphics);
        });
        if let Some(Button::Mouse(_button)) = event.release_args() {
            game_state.request_action(game::Coordinates {
                x: (cursor[0] * game::GAME_LENGTH as f64 / SCREEN_WIDTH) as usize,
                y: (cursor[1] * game::GAME_LENGTH as f64 / SCREEN_HEIGHT) as usize,
            });
        }
        event.mouse_cursor(|x, y| { cursor = [x, y]; });
    }
}
