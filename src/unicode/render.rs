use crate::unicode::codes::FULL_BLOCK;
use std::cmp;

use super::{
    codes::{
        LEFT_FIVE_EIGHTS_BLOCK, LEFT_HALF_BLOCK, LEFT_ONE_EIGHTS_BLOCK, LEFT_ONE_QUARTERS_BLOCK,
        LEFT_SEVEN_EIGHTS_BLOCK, LEFT_THREE_EIGHTS_BLOCK, LEFT_THREE_QUARTERS_BLOCK,
    },
    colors::Color,
};

pub fn render_loading_bar(
    current: f32,
    min: f32,
    max: f32,
    block_count: i32,
    color: Color,
) -> String {
    let lenght = max - min;
    let progress = current / lenght;

    let amount = progress * block_count as f32;
    let tip_progress = (progress * block_count as f32) % 1.0;

    let mut bar_content = "".to_owned();

    for _ in 0..cmp::min(amount as i32, block_count - 1) {
        bar_content += FULL_BLOCK;
    }

    bar_content += &render_single_block(tip_progress);
    format!("\x1b[3{color}m{bar_content}\x1b[m")
}

pub fn render_single_block(progress: f32) -> String {
    match progress {
        x if x >= (7.0 / 8.0) => FULL_BLOCK.to_owned(),
        x if x >= (6.0 / 8.0) => LEFT_SEVEN_EIGHTS_BLOCK.to_owned(),
        x if x >= (5.0 / 8.0) => LEFT_THREE_QUARTERS_BLOCK.to_owned(),
        x if x >= (4.0 / 8.0) => LEFT_FIVE_EIGHTS_BLOCK.to_owned(),
        x if x >= (3.0 / 8.0) => LEFT_HALF_BLOCK.to_owned(),
        x if x >= (2.0 / 8.0) => LEFT_THREE_EIGHTS_BLOCK.to_owned(),
        x if x >= (1.0 / 8.0) => LEFT_ONE_QUARTERS_BLOCK.to_owned(),
        _ => LEFT_ONE_EIGHTS_BLOCK.to_owned(),
    }
}
