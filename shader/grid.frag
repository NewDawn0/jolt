#version 300 es
precision highp float;

uniform vec2 u_offset;
uniform float u_grid_size;

out vec4 outCol;

// Helper to draw grid at specific thickness
float gridLevel(vec2 uv, float scale, float thickness) {
    vec2 grid = fract(uv / scale);
    // Derivative = change per pixel
    vec2 drv = fwidth(uv / scale);

    // thickness * derivative to get screen-space width
    vec2 line = smoothstep(drv * thickness, vec2(0.0), grid) +
                smoothstep(drv * thickness, vec2(0.0), 1.0 - grid);
    return max(line.x, line.y);
}

void main() {
    vec2 uv = (gl_FragCoord.xy - u_offset);
    // Get zooming tier
    float log5 = log(1.0 / 1.0) / log(5.0);
    float floorLog = floor(log5);
    float fractLog = fract(log5);
    // Get dynamic scales for the 3 layers
    float scaleMajor = u_grid_size * pow(5.0, -floorLog + 1.0);
    float scaleMed   = u_grid_size * pow(5.0, -floorLog);
    float scaleMinor = u_grid_size * pow(5.0, -floorLog - 1.0);
    // Render layers with their thickness
    float v1 = gridLevel(uv, scaleMajor, 2.5);
    float v2 = gridLevel(uv, scaleMed,   1.5);
    float v3 = gridLevel(uv, scaleMinor, 1.0);
    // 4. Enhanced Fading Logic
    // Minor grid fades as we zoom out
    float fadeMinor = smoothstep(0.1, 0.9, fractLog);
    // Medium lines transition into major or minor lines depending on zoom
    float medStrength = mix(0.4, 1.0, 1.0 - fractLog);
    // Combine layers
    // The major lines (v1) are always 100% visible
    // The medium lines (v2) fade into major/minor lines
    // The minor lines (v3) fade out on zoom-out
    float grid = v1 * 1.0 + (v2 * medStrength * 0.6) + (v3 * fadeMinor * 0.3);
    // Return out
    outCol = mix(vec4(0.2, 0.2, 0.2, 1.0), vec4(0.9, 0.8, 0.9, 1.0), clamp(grid, 0.0, 1.0));
}
