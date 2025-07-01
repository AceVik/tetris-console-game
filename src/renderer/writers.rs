use super::consts::*;
use crate::game::consts::field::{Area, Row, FIELD_WIDTH};
use crate::game::consts::qube::{Qube, QUBE_SIZE};
use crate::game::consts::qubes::EMPTY_QUBE_ROW;
use crate::game::geometry::Direction;
use crate::game::object::Object;
use crate::game::tetromino::Tetromino;
use crate::game::Game;
use crate::{empty_block, filled_block};
use crossterm::cursor::MoveTo;
use crossterm::style::Color::{Red, White};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, SetSize};
use crossterm::{cursor, execute, queue};
use std::io;
use std::io::Write;

#[inline(always)]
pub fn n_write(out: &mut impl Write, c: char, count: usize) -> io::Result<()> {
    for _ in 0..count {
        write!(out, "{}", c)?;
    }
    Ok(())
}

#[inline(always)]
pub fn write_field_row(out: &mut impl Write, row: Row) -> io::Result<()> {
    for i in (0..FIELD_WIDTH).rev() {
        if row & (1 << i) != 0 {
            execute!(out, SetForegroundColor(FILLED_BLOCK_COLOR))?;
            write!(out, "{}", filled_block!())?;
        } else {
            // Reset color
            execute!(out, SetForegroundColor(EMPTY_BLOCK_COLOR))?;
            write!(out, "{}", empty_block!())?;
        }
    }
    Ok(())
}

#[inline(always)]
pub fn write_field(out: &mut impl Write, field: &Area) -> io::Result<()> {
    for (y, row) in field.iter().enumerate() {
        execute!(out, MoveTo(H_OFFSET + 1, V_OFFSET + y as u16 + 1))?;
        write_field_row(out, *row)?;
    }

    Ok(())
}

#[inline(always)]
pub fn size_screen(out: &mut impl Write) -> io::Result<()> {
    execute!(out, SetSize(SCREEN_WIDTH, SCREEN_HEIGHT))?;
    Ok(())
}

#[inline(always)]
pub fn clear_screen(out: &mut impl Write) -> io::Result<()> {
    execute!(
        out,
        SetSize(SCREEN_WIDTH, SCREEN_HEIGHT),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::White),
        cursor::Hide,
        Clear(ClearType::All)
    )?;

    Ok(())
}

/// Writes the top border by chaining write operations sequentially.
#[inline(always)]
pub fn write_top_border(out: &mut impl Write) -> io::Result<()> {
    execute!(out, MoveTo(H_OFFSET, V_OFFSET))?;
    write!(out, "{}", BORDER_TOP_LEFT)?;
    n_write(out, BORDER_HORIZONTAL, AREA_WIDTH as usize)?;
    write!(out, "{}", BORDER_TOP_MIDDLE)?;
    n_write(out, BORDER_HORIZONTAL, SIDEBAR_WIDTH as usize)?;
    write!(out, "{}", BORDER_TOP_RIGHT)?;
    Ok(())
}

#[inline(always)]
pub fn write_bottom_border(out: &mut impl Write) -> io::Result<()> {
    execute!(out, MoveTo(H_OFFSET, CONTAINER_HEIGHT + V_OFFSET - 1))?;
    write!(out, "{}", BORDER_BOTTOM_LEFT)?;
    n_write(out, BORDER_HORIZONTAL, AREA_WIDTH as usize)?;
    write!(out, "{}", BORDER_BOTTOM_MIDDLE)?;
    n_write(out, BORDER_HORIZONTAL, SIDEBAR_WIDTH as usize)?;
    write!(out, "{}", BORDER_BOTTOM_RIGHT)?;
    Ok(())
}

#[inline(always)]
pub fn write_vertical_borders(out: &mut impl Write) -> io::Result<()> {
    for y in V_OFFSET + 1..CONTAINER_HEIGHT + V_OFFSET - 1 {
        execute!(out, MoveTo(H_OFFSET, y))?;
        write!(out, "{}", BORDER_VERTICAL)?;
        execute!(out, MoveTo(H_OFFSET + AREA_WIDTH + 1, y))?;
        write!(out, "{}", BORDER_VERTICAL_SINGLE)?;
        execute!(out, MoveTo(H_OFFSET + AREA_WIDTH + SIDEBAR_WIDTH + 2, y))?;
        write!(out, "{}", BORDER_VERTICAL)?;
    }

    Ok(())
}

pub fn get_tetromino_color(tetromino: &Tetromino) -> Color {
    match tetromino {
        Tetromino::I => Color::Cyan,
        Tetromino::J => Color::Blue,
        Tetromino::L => Color::Yellow,
        Tetromino::O => Color::Red,
        Tetromino::S => Color::Green,
        Tetromino::T => Color::Magenta,
        Tetromino::Z => Color::White,
    }
}

#[inline(always)]
pub fn write_current_object(out: &mut impl Write, obj: &Object) -> io::Result<()> {
    // Set object color and position
    let color = get_tetromino_color(&obj.tetromino);
    execute!(out, SetForegroundColor(color))?;

    for (y, row) in obj.qube.iter().enumerate() {
        if *row == EMPTY_QUBE_ROW {
            continue;
        }

        let pos_y = obj.pos.y + y as u16 + V_OFFSET + 1;
        let mut pos_x = obj.pos.x * 2 + H_OFFSET + 1;

        for i in (4..QUBE_SIZE + 4).rev() {
            execute!(out, MoveTo(pos_x, pos_y))?;
            if *row & (1 << i) != 0 {
                write!(out, "{}", filled_block!())?;
            }
            pos_x += 2;
        }
    }

    Ok(())
}

#[inline(always)]
pub fn render_qube(
    out: &mut impl Write,
    qube: &Qube,
    color: Color,
    pos: (u16, u16),
) -> io::Result<()> {
    let mut x = pos.0;
    let mut y = pos.1;

    for row in qube.iter() {
        for i in (4..QUBE_SIZE + 4).rev() {
            queue!(out, MoveTo(x, y))?;
            if *row & (1 << i) != 0 {
                execute!(out, SetForegroundColor(color), Print(filled_block!()))?;
            } else {
                execute!(
                    out,
                    SetForegroundColor(EMPTY_BLOCK_COLOR),
                    Print(empty_block!())
                )?;
            }

            x += 2;
        }

        x = pos.0;
        y += 1;
    }

    Ok(())
}

#[inline(always)]
pub fn write_next_object(
    out: &mut impl Write,
    tetromino: &Tetromino,
    direction: Direction,
) -> io::Result<()> {
    let color = get_tetromino_color(tetromino);
    let start_x = H_OFFSET + AREA_WIDTH + 4;
    let start_y = V_OFFSET + 1;

    execute!(
        out,
        SetForegroundColor(FILLED_BLOCK_COLOR),
        MoveTo(start_x, start_y),
        Print("next")
    )?;

    render_qube(
        out,
        &tetromino.get_cube_by_direction(&direction),
        color,
        (H_OFFSET + AREA_WIDTH + 2, V_OFFSET + 2),
    )?;

    Ok(())
}

#[inline(always)]
fn write_centered_str(
    out: &mut impl Write,
    (text, width): (&str, u16),
    (x, y): (u16, u16),
) -> io::Result<()> {
    let x = x + (width - text.len() as u16) / 2;
    execute!(out, MoveTo(x, y), Print(text))?;
    Ok(())
}

#[inline(always)]
fn write_score(out: &mut impl Write, score: u64) -> io::Result<()> {
    let (x, y) = (H_OFFSET + AREA_WIDTH + 2, V_OFFSET + 7);
    let score_str = format!("{}", score);

    execute!(
        out,
        MoveTo(x + 1, y),
        SetForegroundColor(Color::White),
        Print("Score"),
        SetForegroundColor(Color::Green),
    )?;

    write_centered_str(out, (score_str.as_str(), SIDEBAR_WIDTH), (x, y + 1))?;

    Ok(())
}

#[inline(always)]
fn write_level(out: &mut impl Write, level: u8) -> io::Result<()> {
    let (x, y) = (H_OFFSET + AREA_WIDTH + 2, V_OFFSET + 10);
    let level_str = format!("{}", level);

    execute!(
        out,
        MoveTo(x + 1, y),
        SetForegroundColor(Color::White),
        Print("Level"),
        SetForegroundColor(Color::Green),
    )?;

    write_centered_str(out, (level_str.as_str(), SIDEBAR_WIDTH), (x, y + 1))?;

    Ok(())
}

#[inline(always)]
fn write_centered_modal(out: &mut impl Write, width: u16, height: u16) -> io::Result<()> {
    let x = (SCREEN_WIDTH - width) / 2;
    let y = (SCREEN_HEIGHT - height) / 2;

    n_write(out, ' ', width as usize)?;
    for i in 0..height {
        execute!(out, MoveTo(x, y + i))?;
        n_write(out, ' ', width as usize)?;
    }

    Ok(())
}

const GG_ASCII_ART: [&str; 5] = [
    "  _______  _______",
    " /  ____/ /  ____/",
    "|  |  __ |  |  __ ",
    "|  |_|  ||  |_|  |",
    " \\______/ \\______/",
];

const PAUSE_ASCII_ART: [&str; 3] = [
    "┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐",
    "│P│ │A│ │U│ │S│ │E│",
    "└─┘ └─┘ └─┘ └─┘ └─┘",
];

const CONTROLS_TEXT: [&str; 8] = [
    "R      - rotate",
    "A      - move left",
    "D      - move right",
    "S      - move down",
    "W      - drop down",
    "Space  - pause / resume",
    "Bspce  - restart",
    "Ctrl+C - exit",
];

#[inline(always)]
fn write_pause_content(
    out: &mut impl Write,
    modal_width: u16,
    modal_height: u16,
) -> io::Result<()> {
    let modal_x = (SCREEN_WIDTH.saturating_sub(modal_width)) / 2;
    let modal_y = (SCREEN_HEIGHT.saturating_sub(modal_height)) / 2;

    let content_height = PAUSE_ASCII_ART.len() as u16 + 2 + CONTROLS_TEXT.len() as u16;

    let start_y_content = modal_y + (modal_height.saturating_sub(content_height)) / 2;

    let mut current_y = start_y_content;

    execute!(out, SetForegroundColor(Color::Yellow))?;
    for line in PAUSE_ASCII_ART.iter() {
        let line_len = line.len() as u16;
        let x = modal_x + (modal_width.saturating_sub(line_len)) / 2;
        execute!(out, MoveTo(x + 3, current_y))?;
        write!(out, "{}", line)?;
        current_y += 1;
    }
    execute!(out, SetForegroundColor(Color::Reset))?;

    current_y += 2;

    execute!(out, SetForegroundColor(Color::White))?;
    for line in CONTROLS_TEXT.iter() {
        execute!(out, MoveTo(modal_x + 3, current_y))?;
        write!(out, "{}", line)?;
        current_y += 1;
    }
    execute!(out, SetForegroundColor(Color::Reset))?;

    Ok(())
}

#[inline(always)]
fn write_game_over_content(
    out: &mut impl Write,
    game: &Game,
    modal_width: u16,
    modal_height: u16,
) -> io::Result<()> {
    let modal_x = (SCREEN_WIDTH.saturating_sub(modal_width)) / 2;
    let modal_y = (SCREEN_HEIGHT.saturating_sub(modal_height)) / 2;

    let score_text = format!("Score: {}", game.score);
    let level_text = format!("Level: {}", game.level);

    let content_height = GG_ASCII_ART.len() as u16 + 1 + 1 + 1 + 1;

    let start_y_content = modal_y + (modal_height.saturating_sub(content_height)) / 2 - 2;

    let mut current_y = start_y_content;

    for line in GG_ASCII_ART.iter() {
        let line_len = line.len() as u16;
        let x = modal_x + (modal_width.saturating_sub(line_len)) / 2;
        execute!(out, SetForegroundColor(Red), MoveTo(x, current_y))?;
        write!(out, "{}", line)?;
        current_y += 1;
    }

    current_y += 2;

    let score_len = score_text.len() as u16;
    let x = modal_x + (modal_width.saturating_sub(score_len)) / 2;
    execute!(out, SetForegroundColor(Color::Cyan), MoveTo(x, current_y))?;
    write!(out, "{}", score_text)?;

    current_y += 1;
    let level_len = level_text.len() as u16;
    let x = modal_x + (modal_width.saturating_sub(level_len)) / 2;
    execute!(out, MoveTo(x - 1, current_y))?;
    write!(out, "{}", level_text)?;

    current_y += 3;
    let text = "BACKSPACE to restart";
    let x = modal_x + (modal_width.saturating_sub(text.len() as u16)) / 2;
    execute!(out, SetForegroundColor(White), MoveTo(x, current_y))?;
    write!(out, "{}", text)?;

    current_y += 1;
    let text = "Ctrl + C to exit";
    let x = modal_x + (modal_width.saturating_sub(text.len() as u16)) / 2;
    execute!(out, MoveTo(x, current_y))?;
    write!(out, "{}", text)?;

    out.flush()?;

    Ok(())
}

pub fn write_game(out: &mut impl Write, game: &Game) -> io::Result<()> {
    write_field(out, &game.field.area)?;
    write_current_object(out, &game.current_object)?;
    write_next_object(out, &game.next_object.0, game.next_object.1)?;
    write_score(out, game.score)?;
    write_level(out, game.level)?;

    if game.game_over {
        let modal_width = (SCREEN_WIDTH as f32 * 0.8) as u16;
        let modal_height = (SCREEN_HEIGHT as f32 * 0.7) as u16;
        write_centered_modal(out, modal_width, modal_height)?;
        write_game_over_content(out, game, modal_width, modal_height)?;
    }

    if game.is_paused() {
        let modal_width = (SCREEN_WIDTH as f32 * 0.8) as u16;
        let modal_height = (SCREEN_HEIGHT as f32 * 0.8) as u16;

        write_centered_modal(out, modal_width, modal_height)?;
        write_pause_content(out, modal_width, modal_height)?;
    }

    Ok(())
}

pub fn write_border(out: &mut impl Write) -> io::Result<()> {
    write_top_border(out)?;
    write_bottom_border(out)?;
    write_vertical_borders(out)?;

    Ok(())
}
