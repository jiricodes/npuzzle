// inclusive
#[inline]
pub fn is_in_bounds(x: i32, y: i32, left_top: (i32, i32), right_bot: (i32, i32)) -> bool {
    x >= left_top.0 && x <= right_bot.0 && y >= left_top.1 && y <= right_bot.1
}
