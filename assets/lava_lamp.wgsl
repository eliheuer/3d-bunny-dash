// ======================================================
//  THE LAVA LAMP BACKGROUND SHADER!
// ======================================================
// This little program does NOT run on the regular
// computer brain (the CPU) — it runs on the GRAPHICS
// CARD (the GPU), which runs it for EVERY SINGLE DOT
// on the screen, millions of times, all at once!
//
// It paints swirly rainbow blobs using only sine waves —
// the same wiggly sin() math that makes our bad guy
// bounce. Lots of waves added together = lava lamp!

#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // "uv" is WHERE we are on the background:
    // (0,0) is one corner, (1,1) is the other.
    // Multiplying by 10 makes the pattern repeat more.
    let spot = in.uv * 10.0;

    // The clock! It ticks up forever, which is what
    // makes the pattern MOVE and swirl.
    let t = globals.time * 0.4;

    // FOUR sine waves, each wiggling a different way:
    let wave1 = sin(spot.x + t);                    // side to side
    let wave2 = sin(spot.y + t * 1.3);              // up and down
    let wave3 = sin(spot.x + spot.y + t * 1.7);     // diagonal
    let wave4 = sin(length(spot - vec2(5.0, 4.0)) * 1.4 - t * 2.0); // rings!

    // ADD the waves together and shrink back down.
    // Adding waves makes blobby, cloudy shapes.
    let blob = (wave1 + wave2 + wave3 + wave4) / 4.0;

    // Turn the blob number into a COLOR. We use three
    // more sine waves — one each for red, green, blue —
    // each shifted a third of the way around a circle,
    // so the colors chase each other like a rainbow.
    let red   = 0.5 + 0.5 * sin(3.14159 * blob + t);
    let green = 0.5 + 0.5 * sin(3.14159 * blob + t + 2.09);
    let blue  = 0.5 + 0.5 * sin(3.14159 * blob + t + 4.18);

    // Times 0.6 so the background stays soft and dreamy
    // and doesn't fight with the bunny for attention.
    return vec4<f32>(red * 0.6, green * 0.6, blue * 0.6, 1.0);
}
