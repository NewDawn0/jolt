#version 300 es
precision highp float;

uniform vec2 u_viewport;
uniform vec2 u_position;

out vec4 fragCol;

void main() {
    // 1cm at 96 DPI ≈ 38 pixels
    float cm_to_px = 38.0;

    // Get pixel position relative to viewport and position offset
    vec2 pixel_pos = gl_FragCoord.xy - u_position;

    // Calculate grid line positions (1cm intervals)
    vec2 grid = mod(pixel_pos, cm_to_px);

    // Grid line thickness (2 pixels)
    float line_width = 2.0;

    // Check if we're on a grid line
    bool on_horizontal_line = grid.y < line_width || grid.y > (cm_to_px - line_width);
    bool on_vertical_line = grid.x < line_width || grid.x > (cm_to_px - line_width);

    // Background color (dark gray)
    vec3 bg_color = vec3(0.1, 0.1, 0.1);

    // Grid line color (light gray)
    vec3 grid_color = vec3(0.5, 0.5, 0.5);

    // Mix based on whether we're on a grid line
    if (on_horizontal_line || on_vertical_line) {
        fragCol = vec4(grid_color, 1.0);
    } else {
        fragCol = vec4(bg_color, 1.0);
    }
}
