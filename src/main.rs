use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

use crossbow_android::*;

#[macroquad::main("project_name")]
async fn main() {
  let crossbow = CrossbowInstance::new();
  // info!("initializing play core");
  // let play_core: play_core::PlayCorePlugin = crossbow.get_plugin().unwrap();
  info!("initializing admob");
  let admob: admob_android::AdMobPlugin = crossbow.get_plugin().unwrap();

  let skin = get_skin();
  let window_skin = skin.clone();

  loop {
    clear_background(WHITE);

    root_ui().push_skin(&window_skin);
    root_ui().window(hash!(), vec2(0.0, 50.0), vec2(1000.0, 1000.0), |ui| {
        if ui.button(vec2(-15.0, 250.0), "Show ad") {
            if !admob.is_initialized().unwrap() {
                println!("Calling AdMob::initialize()");
                admob.initialize(true, "G", false, true).unwrap();
            }
            if admob.is_initialized().unwrap() && !admob.is_interstitial_loaded().unwrap() {
                println!("Calling load_interstitial()");
                admob.load_interstitial("ca-app-pub-3940256099942544/1033173712").unwrap();
            }
            if admob.is_interstitial_loaded().unwrap() {
                println!("Calling show_interstitial()");
                admob.show_interstitial().unwrap();
            }
        }
    });
    root_ui().pop_skin();

    // draw_circle(100., 100., 100., BLUE);

    next_frame().await;
  }
}

fn get_skin() -> Skin {
  let label_style = root_ui()
      .style_builder()
      .text_color(Color::from_rgba(180, 180, 120, 255))
      .font_size(30)
      .build();
  let window_style = root_ui()
      .style_builder()
      .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
      .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
      .build();
  let button_style = root_ui()
      .style_builder()
      .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
      .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
      .text_color(Color::from_rgba(180, 180, 100, 255))
      .font_size(40)
      .build();
  let editbox_style = root_ui()
      .style_builder()
      .background_margin(RectOffset::new(0., 0., 0., 0.))
      .text_color(Color::from_rgba(120, 120, 120, 255))
      .color_selected(Color::from_rgba(190, 190, 190, 255))
      .font_size(50)
      .build();
  Skin {
      editbox_style,
      window_style,
      button_style,
      label_style,
      ..root_ui().default_skin()
  }
}