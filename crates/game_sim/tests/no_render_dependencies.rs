use game_sim::SimPlugin;

/// The simulation must not depend on windowing or rendering (ADR-0005): the
/// headless app builds with only the sim plugin and runs ticks.
#[test]
fn runs_headless_without_window() {
    let mut app = bevy::app::App::new();
    app.add_plugins((
        bevy::app::TaskPoolPlugin::default(),
        bevy::time::TimePlugin,
        SimPlugin,
    ))
    .insert_resource(bevy::time::TimeUpdateStrategy::FixedTimesteps(1));
    app.finish();
    app.cleanup();
    app.update();
    for _ in 0..10 {
        app.update();
    }
}
