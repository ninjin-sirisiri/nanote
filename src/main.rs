#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::*;
use gpui_component::{button::*, *};

pub struct HelloWorld;

impl Render for HelloWorld {
  fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
    div()
      .v_flex()
      .gap_2()
      .size_full()
      .items_center()
      .justify_center()
      .child("Hello, World!")
      .child(
        Button::new("ok")
          .primary()
          .label("Let's Go!")
          .on_click(|_, _, _| println!("Clicked!")),
      )
  }
}

fn main() {
  let app = Application::new().with_assets(gpui_component_assets::Assets);

  app.run(move |cx| {
    // This must be called before using any GPUI Component features.
    gpui_component::init(cx);

    cx.spawn(async move |cx| {
      cx.open_window(
        WindowOptions {
          window_bounds: Some(WindowBounds::Maximized(Bounds {
            origin: Point {
              x: Pixels::from(400.),
              y: Pixels::from(300.),
            },
            size: gpui::Size {
              width: Pixels::from(800.),
              height: Pixels::from(600.),
            },
          })),
          titlebar: Some(TitlebarOptions {
            title: Some(SharedString::from("nanote")),
            appears_transparent: true,
            traffic_light_position: None,
          }),
          ..WindowOptions::default()
        },
        |window, cx| {
          let view = cx.new(|_| HelloWorld);
          // This first level on the window, should be a Root.
          cx.new(|cx| Root::new(view, window, cx))
        },
      )?;

      Ok::<_, anyhow::Error>(())
    })
    .detach();
  });
}
