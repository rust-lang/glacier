fn next_letter_position(&self, x_index: f32, y_index: f32, width: f32) -> (f32, f32) {
    let vwidth = &self.viewport.x / 2.0;

    match x_index * width {
        0.0 ... vwidth => {
            (x_index + 1.0, y_index)
        },
        _ => (0.0, y_index - 1.0)
    }
}
