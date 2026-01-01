use typst_world::{TypstWorld, layout_content};
use vello::Scene;
use vello::kurbo::Affine;
use velotyp::TypstScene;
use velotyp::typst_library::diag::{EcoString, SourceResult};
use velotyp::typst_library::foundations::NativeElement;
use velotyp::typst_library::layout::{
    Abs, BoxElem, Frame, Length, Ratio, Rel, Sides,
};
use velotyp::typst_library::text::TextElem;
use velotyp::typst_library::visualize::{Color, Paint};
use winit::event_loop::EventLoop;
use with_winit::{VelloDemo, VelloWinitApp};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = VelloWinitApp::new(WinitDemo::default());

    event_loop.run_app(&mut app).unwrap();
}

#[derive(Default)]
struct WinitDemo {
    world: TypstWorld,
    elapsed: f64,
}

impl WinitDemo {
    fn layout_text(
        &self,
        text: impl Into<EcoString>,
    ) -> SourceResult<Frame> {
        let content = TextElem::new(text.into()).pack();
        let content = BoxElem::new()
            .with_body(Some(content))
            .with_fill(Some(Paint::Solid(Color::WHITE)))
            .with_inset(Sides::splat(Some(Rel::new(
                Ratio::zero(),
                Length {
                    abs: Abs::pt(10.0),
                    ..Default::default()
                },
            ))))
            .pack();

        layout_content(&self.world, &content)
    }
}

impl VelloDemo for WinitDemo {
    fn window_title(&self) -> &'static str {
        "Velotyp Winit Showcase"
    }
    fn initial_logical_size(&self) -> (f64, f64) {
        (800.0, 600.0)
    }

    fn rebuild_scene(
        &mut self,
        scene: &mut Scene,
        _scale_factor: f64,
    ) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let text = format!(
            "Hello from Typst!\nRunning at {:.2}fps!",
            1.0 / (now - self.elapsed)
        );

        let frame = self
            .layout_text(text)
            .expect("Should be able to compile.");
        let new_scene = TypstScene::new(&frame).render();

        scene.append(&new_scene, Some(Affine::scale(4.0)));

        self.elapsed = now;
    }
}
