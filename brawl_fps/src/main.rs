use macroquad::prelude::*;

mod nivel1;
mod nivel2;

enum MenuState {
    Main,
    Level1,
    Level2,
}

#[macroquad::main("Shrek Find and Rescue")]
async fn main() {
    let title_tex: Texture2D = load_texture("img/menu.png").await.unwrap();
    title_tex.set_filter(FilterMode::Nearest);

    let mut state = MenuState::Main;

    loop {
        clear_background(BLACK);

        match state {
            MenuState::Main => {
                // Dibuja la imagen de fondo/menú escalada a la pantalla
                draw_texture_ex(
                    &title_tex,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );

                // ===== configuración de botones =====
                // Puedes ajustar btn_w, btn_h y spacing si quieres otro tamaño/espaciado.
                let btn_w = 220.0;
                let btn_h = 56.0;
                let spacing = 18.0;

                // Factor horizontal para situar los botones "al lado derecho" de la imagen.
                // 0.75 -> centro-derecha; sube a 0.8 para moverlos más a la derecha, baja a 0.65 para acercarlos al centro.
                let right_x_factor = 0.75_f32;
                let center_x = screen_width() * right_x_factor;

                // calcular posición Y para centrar los 3 botones verticalmente
                let total_h = btn_h * 3.0 + spacing * 2.0;
                let start_y = screen_height() / 2.0 - total_h / 2.0;

                let btn1_x = center_x - btn_w / 2.0;
                let btn1_y = start_y;
                let btn2_x = btn1_x;
                let btn2_y = btn1_y + btn_h + spacing;
                let btn3_x = btn1_x;
                let btn3_y = btn2_y + btn_h + spacing;

                let button1 = Rect::new(btn1_x, btn1_y, btn_w, btn_h);
                let button2 = Rect::new(btn2_x, btn2_y, btn_w, btn_h);
                let button3 = Rect::new(btn3_x, btn3_y, btn_w, btn_h);

                // Colores: azul, morado (personalizado), rojo
                let color_btn1 = DARKBLUE;
                let color_btn2 = Color::new(0.55, 0.15, 0.6, 1.0); // morado
                let color_btn3 = RED;

                // hover color (ligero aclarado)
                let hover_color = Color::new(0.85, 0.85, 0.85, 1.0);

                // dibujar sombras/contornos si el mouse está encima (pequeño efecto)
                let (mx, my) = mouse_position();

                // Botón 1
                if button1.contains(vec2(mx, my)) {
                    draw_rectangle(button1.x - 6.0, button1.y - 6.0, button1.w + 12.0, button1.h + 12.0, hover_color);
                } else {
                    draw_rectangle(button1.x - 2.0, button1.y - 2.0, button1.w + 4.0, button1.h + 4.0, DARKGRAY);
                }
                draw_rectangle(button1.x, button1.y, button1.w, button1.h, color_btn1);
                let label1 = "Nivel 1";
                let mt1 = measure_text(label1, None, 30, 1.0);
                draw_text(label1, button1.x + button1.w / 2.0 - mt1.width / 2.0, button1.y + button1.h / 2.0 + 10.0, 30.0, WHITE);

                // Botón 2
                if button2.contains(vec2(mx, my)) {
                    draw_rectangle(button2.x - 6.0, button2.y - 6.0, button2.w + 12.0, button2.h + 12.0, hover_color);
                } else {
                    draw_rectangle(button2.x - 2.0, button2.y - 2.0, button2.w + 4.0, button2.h + 4.0, DARKGRAY);
                }
                draw_rectangle(button2.x, button2.y, button2.w, button2.h, color_btn2);
                let label2 = "Nivel 2";
                let mt2 = measure_text(label2, None, 30, 1.0);
                draw_text(label2, button2.x + button2.w / 2.0 - mt2.width / 2.0, button2.y + button2.h / 2.0 + 10.0, 30.0, WHITE);

                // Botón 3 (Salir)
                if button3.contains(vec2(mx, my)) {
                    draw_rectangle(button3.x - 6.0, button3.y - 6.0, button3.w + 12.0, button3.h + 12.0, hover_color);
                } else {
                    draw_rectangle(button3.x - 2.0, button3.y - 2.0, button3.w + 4.0, button3.h + 4.0, DARKGRAY);
                }
                draw_rectangle(button3.x, button3.y, button3.w, button3.h, color_btn3);
                let label3 = "Salir";
                let mt3 = measure_text(label3, None, 30, 1.0);
                draw_text(label3, button3.x + button3.w / 2.0 - mt3.width / 2.0, button3.y + button3.h / 2.0 + 10.0, 30.0, WHITE);

                // manejar clicks
                if is_mouse_button_pressed(MouseButton::Left) {
                    if button1.contains(vec2(mx, my)) {
                        state = MenuState::Level1;
                    } else if button2.contains(vec2(mx, my)) {
                        state = MenuState::Level2;
                    } else if button3.contains(vec2(mx, my)) {
                        std::process::exit(0);
                    }
                }
            }

            MenuState::Level1 => {
                nivel1::run_level1().await;
                state = MenuState::Main; // volver al menú cuando termine
            }

            MenuState::Level2 => {
                nivel2::run_level2().await;
                state = MenuState::Main;
            }
        }

        next_frame().await;
    }
}
