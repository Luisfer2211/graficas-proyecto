
use macroquad::prelude::*;
use macroquad::texture::FilterMode;
use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound};
use std::time::Duration;
use std::thread::sleep;

// ====== Config ======
const MAP_W: usize = 16;
const MAP_H: usize = 12;
const FOV: f32 = 0.66; // ~66°
const MOVE_SPEED: f32 = 2.5; // velocidad reducida
const MINIMAP_SCALE: f32 = 6.0; // px por celda en minimapa
const MAX_FPS: f32 = 20.0; // cap máximo de FPS
const MOUSE_SENSITIVITY: f32 = 0.003; // sensibilidad reducida

// 0 = vacío, 1 = pared verde, 2 = moneda, 3 = salida roja
static mut MAP: [[i32; MAP_W]; MAP_H] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,2,0,0,0,0,0,1,0,0,0,0,0,0,0,1],
    [1,0,1,1,1,1,0,1,0,1,1,1,1,1,3,1],
    [1,0,1,0,0,0,0,1,0,0,0,0,0,1,1,1],
    [1,0,1,0,1,1,0,1,1,1,0,1,0,0,0,1],
    [1,0,0,0,1,0,0,0,0,1,0,0,0,1,0,1],
    [1,1,1,0,1,0,1,1,0,1,1,1,0,1,0,1],
    [1,0,0,0,0,0,1,4,0,0,0,1,0,0,0,1],
    [1,0,1,1,1,0,1,1,1,1,0,1,1,1,0,1],
    [1,0,0,0,1,0,0,0,0,0,0,0,0,1,0,1],
    [1,0,1,0,1,1,1,1,1,1,1,1,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

#[derive(Clone, Copy)]
struct Camera {
    pos: Vec2,
    dir: Vec2,
    plane: Vec2,
}

impl Camera {
    fn new() -> Self {
        let pos = vec2(1.5, 10.5);
        let dir = vec2(1.0, 0.0);
        let plane = vec2(0.0, FOV);
        Self { pos, dir, plane }
    }

    fn rotate(&mut self, angle: f32) {
        let (sin_a, cos_a) = angle.sin_cos();
        let old_dir_x = self.dir.x;
        self.dir.x = self.dir.x * cos_a - self.dir.y * sin_a;
        self.dir.y = old_dir_x * sin_a + self.dir.y * cos_a;
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * cos_a - self.plane.y * sin_a;
        self.plane.y = old_plane_x * sin_a + self.plane.y * cos_a;
    }
}

/// Retorna si una celda es pared (solo tipo 1)
fn is_wall(cell: i32) -> bool {
    cell == 1
}

/// Crea una textura 2x2 RGBA de placeholder con color dado (valores 0..255)
fn make_placeholder_texture(r: u8, g: u8, b: u8) -> Texture2D {
    // 2x2 pixels, RGBA
    let bytes: Vec<u8> = vec![
        r, g, b, 255,
        r, g, b, 255,
        r, g, b, 255,
        r, g, b, 255,
    ];
    Texture2D::from_rgba8(2, 2, &bytes)
}

pub async fn run_level2() {
    // ---- Texturas ----
    let planicie = match load_texture("img/planicie.png").await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/planicie.png: {}. Usando placeholder.", e);
            make_placeholder_texture(10, 200, 100)
        }
    };
    let bosque = match load_texture("img/bosque.png").await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/bosque.png: {}. Usando placeholder.", e);
            make_placeholder_texture(30, 120, 30)
        }
    };
    let castillo = match load_texture("img/castillo.png").await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/castillo.png: {}. Usando placeholder.", e);
            make_placeholder_texture(160, 160, 200)
        }
    };
    let burro = match load_texture("img/burro.png").await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/burro.png: {}. Usando placeholder.", e);
            make_placeholder_texture(240, 200, 50)
        }
    };
    let fiona = match load_texture("img/fiona.png").await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/fiona.png: {}. Usando placeholder.", e);
            make_placeholder_texture(200, 80, 120)
        }
    };

    let gato = match load_texture("img/gato.png").await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/gato.png: {}. Usando placeholder.", e);
            make_placeholder_texture(150, 100, 250)
        }
    };

    gato.set_filter(FilterMode::Linear);
    planicie.set_filter(FilterMode::Linear);
    bosque.set_filter(FilterMode::Linear);
    castillo.set_filter(FilterMode::Linear);
    burro.set_filter(FilterMode::Linear);
    fiona.set_filter(FilterMode::Linear);

    // ---- Audios ----
    // Guardamos las Sound en Option<Sound> (no las movemos fuera; usaremos as_ref() para pasar &Sound).
    let bg_sound_opt: Option<Sound> = match load_sound("img/fondo.wav").await {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/fondo.wav: {}. Audio de fondo deshabilitado.", e);
            None
        }
    };
    let coin_sound_opt: Option<Sound> = match load_sound("img/moneda.wav").await {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/moneda.wav: {}. Sonido de moneda deshabilitado.", e);
            None
        }
    };

    let coin1_sound_opt: Option<Sound> = match load_sound("img/moneda1.wav").await {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/moneda1.wav: {}. Sonido de moneda deshabilitado.", e);
            None
        }
    };

    let final_sound_opt: Option<Sound> = match load_sound("img/final.wav").await {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Warning: no se pudo cargar img/final.wav: {}. Sonido final deshabilitado.", e);
            None
        }
    };

    // Flags para controlar reproducción/pausa/estado final
    let mut bg_playing = false;
    let mut bg_should_play = true; // cuando final suena lo ponemos en false para evitar reinicios
    let mut final_played = false;

    // Iniciamos el audio de fondo en bucle (si está disponible)
    if let Some(bg_ref) = bg_sound_opt.as_ref() {
        play_sound(bg_ref, PlaySoundParams { looped: true, volume: 0.6 });
        bg_playing = true;
    }

    // posiciones clave
    let spawn = vec2(1.5, 10.5);
    let coin_pos_opt = find_first_cell(2);
    let exit_pos_opt = find_first_cell(3);

    let mut cam = Camera::new();
    let mut mouse_look = true;
    set_cursor_grab(true);
    show_mouse(false);
    let mut last_mouse_x = mouse_position().0;

    let mut coins = count_coins();
    let mut won = false;
    let mut paused = false;

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // ====== INPUT ======
        if is_key_pressed(KeyCode::Escape) {
            paused = !paused;

            // Pausamos/Despausamos audio de fondo (simulación: stop/replay)
            if paused {
                // Pausamos: paramos el sonido de fondo si está sonando
                if bg_playing {
                    if let Some(bg_ref) = bg_sound_opt.as_ref() {
                        stop_sound(bg_ref);
                    }
                    bg_playing = false;
                }
                set_cursor_grab(false);
                show_mouse(true);
            } else {
                // Despausamos: volvemos a iniciar el fondo si corresponde y no hemos llegado al final
                if bg_should_play && !bg_playing && !final_played {
                    if let Some(bg_ref) = bg_sound_opt.as_ref() {
                        play_sound(bg_ref, PlaySoundParams { looped: true, volume: 0.6 });
                        bg_playing = true;
                    }
                }
                if mouse_look {
                    set_cursor_grab(true);
                    show_mouse(false);
                }
                last_mouse_x = mouse_position().0;
            }
        }

        if is_key_pressed(KeyCode::M) {
            mouse_look = !mouse_look;
            if mouse_look && !paused {
                set_cursor_grab(true);
                show_mouse(false);
                last_mouse_x = mouse_position().0;
            } else {
                set_cursor_grab(false);
                show_mouse(true);
            }
        }

        // Movimiento/rotación (si no pausado ni ganado)
        if !paused && !won {
            if mouse_look {
                let (mx, _) = mouse_position();
                let dx = mx - last_mouse_x;
                last_mouse_x = mx;
                cam.rotate(dx * MOUSE_SENSITIVITY);
            }

            let move_step = MOVE_SPEED * dt;

            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                let dirc = cam.dir;
                try_move(&mut cam, dirc * move_step, coins);
            }
            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                let dirc = cam.dir;
                try_move(&mut cam, -dirc * move_step, coins);
            }

            // strafing: derecha = (-dir.y, dir.x) ; A = izquierda, D = derecha
            let right = vec2(-cam.dir.y, cam.dir.x);
            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                try_move(&mut cam, -right * move_step, coins);
            }
            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                try_move(&mut cam, right * move_step, coins);
            }
        }

        // Recolección de monedas y condición de salida
        unsafe {
            let cx = cam.pos.x as usize;
            let cy = cam.pos.y as usize;
            if cx < MAP_W && cy < MAP_H {
                if MAP[cy][cx] == 2 {
                    // Recolectada
                    MAP[cy][cx] = 0;
                    coins -= 1;

                    // Reproducir sonido de moneda sin detener el fondo (si está disponible)
                    if let Some(coin_ref) = coin_sound_opt.as_ref() {
                        play_sound(coin_ref, PlaySoundParams { looped: false, volume: 0.95 });
                    }
                }

                if MAP[cy][cx] == 4 {
                    // Recolectada
                    MAP[cy][cx] = 0;
                    coins -= 1;

                    // Reproducir sonido de moneda sin detener el fondo (si está disponible)
                    if let Some(coin1_ref) = coin1_sound_opt.as_ref() {
                        play_sound(coin1_ref, PlaySoundParams { looped: false, volume: 0.95 });
                    }
                }

                if MAP[cy][cx] == 3 && coins == 0 {
                    // Jugador gana: paramos sonido de fondo inmediatamente y reproducimos final (una sola vez)
                    if !final_played {
                        if let Some(bg_ref) = bg_sound_opt.as_ref() {
                            // paramos fondo justo antes de reproducir el final
                            stop_sound(bg_ref);
                            bg_playing = false;
                        }
                        // ya no queremos que el fondo vuelva a iniciarse
                        bg_should_play = false;

                        // reproducir final
                        if let Some(final_ref) = final_sound_opt.as_ref() {
                            play_sound(final_ref, PlaySoundParams { looped: false, volume: 1.0 });
                        }
                        final_played = true;
                    }

                    // marcamos estado de victoria
                    won = true;
                    set_cursor_grab(false);
                    show_mouse(true);
                }
            }
        }

        // RAYCAST: ahora pasamos `coins` para que la celda 3 sea muro solo si quedan monedas.
        let z_buffer = draw_scene(
            &cam,
            &planicie,
            &bosque,
            &castillo,
            spawn,
            coin_pos_opt,
            exit_pos_opt,
            coins,
        );

        // Sprites 3D: monedas (burro) y salida (fiona)
        draw_sprites_3d(&cam, &z_buffer, &burro, &gato, &fiona, coins);

        // Minimap y HUD (ahora draw_minimap recibe coins para mostrar estado dinámico)
        draw_minimap(&cam, coins);
        let fps = get_fps();
        draw_text("Esc para pausar", 10.0, 40.0, 18.0, WHITE);

        if !won {
            let hud = format!("Amigos por encontrar: {} | FPS: {:.0}", coins, fps);
            draw_text(&hud, 10.0, 20.0, 22.0, YELLOW);
        } else {
            let sw = screen_width();
            let sh = screen_height();
            draw_rectangle(0.0, 0.0, sw, sh, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.7 });
            let msg = "¡Has ganado!";
            let tw = measure_text(msg, None, 60, 1.0);
            draw_text(msg, sw/2.0 - tw.width/2.0, sh/2.0, 60.0, GOLD);

            // ---- BOTÓN: Volver al menú cuando has ganado ----
            // posición y tamaño del botón (centrado debajo del mensaje)
            let btn_w = 220.0;
            let btn_h = 48.0;
            let btn_x = sw / 2.0 - btn_w / 2.0;
            let btn_y = sh / 2.0 + 40.0;

            let btn_rect = Rect::new(btn_x, btn_y, btn_w, btn_h);

            // cambiar apariencia si el mouse está encima
            let (mx, my) = mouse_position();
            if btn_rect.contains(vec2(mx, my)) {
                draw_rectangle(btn_x - 4.0, btn_y - 4.0, btn_w + 8.0, btn_h + 8.0, GRAY);
            } else {
                draw_rectangle(btn_x - 2.0, btn_y - 2.0, btn_w + 4.0, btn_h + 4.0, DARKGRAY);
            }

            draw_rectangle(btn_x, btn_y, btn_w, btn_h, DARKBLUE);
            let label = "Volver al menú";
            let lt = measure_text(label, None, 28, 1.0);
            draw_text(label, btn_x + btn_w / 2.0 - lt.width / 2.0, btn_y + btn_h / 2.0 + 10.0, 28.0, WHITE);

            // detectar click en el botón y salir del nivel
            if is_mouse_button_pressed(MouseButton::Left) {
                if btn_rect.contains(vec2(mx, my)) {
                    // asegurar cursor visible antes de salir (ya lo mostramos, por seguridad)
                    set_cursor_grab(false);
                    show_mouse(true);
                    // salir del nivel: main.rs recibirá el control y volverá al menú principal
                    break;
                }
            }
        }

        if paused && !won {
            let sw = screen_width();
            let sh = screen_height();
            draw_rectangle(0.0, 0.0, sw, sh, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 });
            let msg = "Pausado — presiona ESC para continuar";
            let tw = measure_text(msg, None, 32, 1.0);
            draw_text(msg, sw/2.0 - tw.width/2.0, sh/2.0, 32.0, WHITE);

            // ---- BOTÓN: Volver al menú también en pausa ----
            // posición y tamaño del botón (centrado debajo del texto)
            let btn_w = 220.0;
            let btn_h = 48.0;
            let btn_x = sw / 2.0 - btn_w / 2.0;
            let btn_y = sh / 2.0 + 40.0;

            let btn_rect = Rect::new(btn_x, btn_y, btn_w, btn_h);

            // apariencia hover
            let (mx, my) = mouse_position();
            if btn_rect.contains(vec2(mx, my)) {
                draw_rectangle(btn_x - 4.0, btn_y - 4.0, btn_w + 8.0, btn_h + 8.0, GRAY);
            } else {
                draw_rectangle(btn_x - 2.0, btn_y - 2.0, btn_w + 4.0, btn_h + 4.0, DARKGRAY);
            }
            draw_rectangle(btn_x, btn_y, btn_w, btn_h, DARKBLUE);
            let label = "Volver al menú";
            let lt = measure_text(label, None, 28, 1.0);
            draw_text(label, btn_x + btn_w / 2.0 - lt.width / 2.0, btn_y + btn_h / 2.0 + 10.0, 28.0, WHITE);

            // detectar click y salir del nivel (volver al menú)
            if is_mouse_button_pressed(MouseButton::Left) {
                if btn_rect.contains(vec2(mx, my)) {
                    set_cursor_grab(false);
                    show_mouse(true);
                    break;
                }
            }
        }

        // Cap FPS simple
        let target_dt = 1.0 / MAX_FPS;
        if dt < target_dt {
            let to_sleep = target_dt - dt;
            sleep(Duration::from_secs_f32(to_sleep));
        }

        next_frame().await;
    }
}

/// Encuentra la primera celda con el valor `val` (retorna coords como Vec2 del centro de la celda)
fn find_first_cell(val: i32) -> Option<Vec2> {
    unsafe {
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                if MAP[y][x] == val {
                    return Some(vec2(x as f32 + 0.5, y as f32 + 0.5));
                }
            }
        }
    }
    None
}

/// Intentar mover la cámara: chequeo combinado (nx,ny) para evitar "sliding" parcial atravesando paredes.
/// Si la celda de destino es `3` (salida) se permite solo si coins == 0.
fn try_move(cam: &mut Camera, delta: Vec2, coins: i32) {
    let next = cam.pos + delta;

    unsafe {
        let nx = next.x as isize;
        let ny = next.y as isize;
        if nx < 0 || nx as usize >= MAP_W || ny < 0 || ny as usize >= MAP_H {
            return;
        }
        let cell = MAP[ny as usize][nx as usize];
        let blocked = if cell == 3 { coins > 0 } else { is_wall(cell) };
        if !blocked {
            cam.pos = next;
        }
    }
}

fn count_coins() -> i32 {
    unsafe {
        let mut c = 0;
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                if MAP[y][x] == 2 || MAP[y][x] == 4 {
                    c += 1;
                }
            }
        }
        c
    }
}


/// Dibuja paredes texturizadas y devuelve z-buffer (distancia perpendicular por columna).
/// La celda 3 (salida) se considera muro **solo** cuando quedan monedas (coins > 0).
fn draw_scene(
    cam: &Camera,
    planicie: &Texture2D,
    bosque: &Texture2D,
    castillo: &Texture2D,
    spawn: Vec2,
    coin_pos_opt: Option<Vec2>,
    exit_pos_opt: Option<Vec2>,
    coins: i32,
) -> Vec<f32> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh * 0.5, DARKBLUE);
    draw_rectangle(0.0, sh * 0.5, sw, sh * 0.5, DARKGRAY);

    let mut z_buffer = vec![1e30f32; sw as usize];

    for x in 0..(sw as i32) {
        let camera_x = 2.0 * x as f32 / sw - 1.0;
        let ray_dir = vec2(
            cam.dir.x + cam.plane.x * camera_x,
            cam.dir.y + cam.plane.y * camera_x,
        );

        let mut map_x = cam.pos.x as i32;
        let mut map_y = cam.pos.y as i32;

        let delta_dist_x = if ray_dir.x == 0.0 { 1e30 } else { (1.0 / ray_dir.x).abs() };
        let delta_dist_y = if ray_dir.y == 0.0 { 1e30 } else { (1.0 / ray_dir.y).abs() };

        let (step_x, mut side_dist_x) = if ray_dir.x < 0.0 {
            let dist = (cam.pos.x - map_x as f32) * delta_dist_x;
            (-1, dist)
        } else {
            let dist = (map_x as f32 + 1.0 - cam.pos.x) * delta_dist_x;
            (1, dist)
        };
        let (step_y, mut side_dist_y) = if ray_dir.y < 0.0 {
            let dist = (cam.pos.y - map_y as f32) * delta_dist_y;
            (-1, dist)
        } else {
            let dist = (map_y as f32 + 1.0 - cam.pos.y) * delta_dist_y;
            (1, dist)
        };

        let mut hit = false;
        let mut side = 0;
        let mut cell = 0;

        unsafe {
            while !hit {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                if map_x < 0 || map_x >= MAP_W as i32 || map_y < 0 || map_y >= MAP_H as i32 {
                    break;
                }

                cell = MAP[map_y as usize][map_x as usize];
                // Ahora: la celda 3 (salida) bloquea sólo si quedan monedas.
                let is_blocking = cell == 1 || (cell == 3 && coins > 0);
                if is_blocking {
                    hit = true;
                }
            }
        }

        if hit {
            let perp_wall_dist = if side == 0 {
                (map_x as f32 - cam.pos.x + (1 - step_x) as f32 / 2.0) / ray_dir.x
            } else {
                (map_y as f32 - cam.pos.y + (1 - step_y) as f32 / 2.0) / ray_dir.y
            };

            if perp_wall_dist > 0.0 {
                z_buffer[x as usize] = perp_wall_dist;
            }

            let line_h = (sh / perp_wall_dist.max(0.0001)).round();
            let draw_start = ((-line_h / 2.0) + sh / 2.0).max(0.0);

            // wallX: posición fraccional en la pared (0..1)
            let mut wall_x = if side == 0 {
                cam.pos.y + perp_wall_dist * ray_dir.y
            } else {
                cam.pos.x + perp_wall_dist * ray_dir.x
            };
            wall_x -= wall_x.floor();

            // Si es la celda salida y todavía quedan monedas, la textura será siempre castillo
            let tex: &Texture2D = if cell == 3 && coins > 0 {
                castillo
            } else {
                // textura elegida por cercanía a spawn/coin/exit (tu lógica original)
                let bx = map_x as f32 + 0.5;
                let by = map_y as f32 + 0.5;
                let ds = (bx - spawn.x).hypot(by - spawn.y);
                let dc = coin_pos_opt.map(|p| (bx - p.x).hypot(by - p.y)).unwrap_or(f32::INFINITY);
                let de = exit_pos_opt.map(|p| (bx - p.x).hypot(by - p.y)).unwrap_or(f32::INFINITY);

                if ds <= dc && ds <= de {
                    planicie
                } else if dc <= ds && dc <= de {
                    bosque
                } else {
                    castillo
                }
            };

            // muestreo: tex_x en pixels (clampeado)
            let tex_w = tex.width().max(1.0);
            let tex_h = tex.height().max(1.0);
            let mut tex_x = wall_x * tex_w;
            if tex_x < 0.0 { tex_x = 0.0; }
            if tex_x >= tex_w { tex_x = tex_w - 1.0; }

            let source = Some(Rect::new(tex_x, 0.0, 1.0, tex_h));
            let dest_size = Some(vec2(1.0, line_h));
            let params = DrawTextureParams {
                dest_size,
                source,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            };
            draw_texture_ex(tex, x as f32, draw_start, WHITE, params);
        }
    }

    z_buffer
}

/// Dibuja monedas y la salida en 3D como sprites (texturas), respetando z-buffer.
/// La salida solo se dibuja como sprite si coins == 0 (después de recoger).
/// La animación de la salida usa un bob sin() para subir/bajar.
fn draw_sprites_3d(
    cam: &Camera,
    z_buffer: &Vec<f32>,
    coin_tex: &Texture2D,   // textura para moneda tipo 2 (burro)
    gato_tex: &Texture2D,   // textura para moneda tipo 4 (gato)
    exit_tex: &Texture2D,   // textura para la salida (fiona)
    coins: i32
) {
    let sw = screen_width();
    let sh = screen_height();
    let inv_det = 1.0 / (cam.plane.x * cam.dir.y - cam.dir.x * cam.plane.y);
    let t = get_time() as f32;

    unsafe {
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                let cell = MAP[y][x];

                // Si es la salida pero no hemos recogido las monedas -> no dibujar sprite (esa celda se ve como pared)
                if cell == 3 && coins > 0 {
                    continue;
                }

                // Dibujar solo si es moneda (2 o 4) o salida (3)
                if cell == 2 || cell == 3 || cell == 4 {
                    let sprite_x = (x as f32 + 0.5) - cam.pos.x;
                    let sprite_y = (y as f32 + 0.5) - cam.pos.y;

                    let transform_x = inv_det * (cam.dir.y * sprite_x - cam.dir.x * sprite_y);
                    let transform_y = inv_det * (-cam.plane.y * sprite_x + cam.plane.x * sprite_y);

                    if transform_y <= 0.0 { continue; }

                    let screen_x = (sw / 2.0) * (1.0 + transform_x / transform_y);

                    let mut sprite_h = (sh / transform_y).abs();
                    // escala diferente para monedas vs salida
                    sprite_h *= if cell == 2 || cell == 4 { 0.45 } else { 0.85 };
                    let sprite_w = sprite_h;

                    let draw_start_y = (sh / 2.0) - (sprite_h / 2.0);
                    let draw_start_x = screen_x - (sprite_w / 2.0);

                    let center_column = screen_x as isize;
                    if center_column < 0 || (center_column as usize) >= z_buffer.len() { continue; }
                    if transform_y >= z_buffer[center_column as usize] { continue; }

                    // bob vertical - para fiona (salida) y un poco para las monedas también
                    let bob = if cell == 3 {
                        // salida: movimiento vertical más pronunciado
                        (t * 2.4).sin() * (sprite_h * 0.08)
                    } else {
                        // monedas: leve bob para dar vida
                        (t * 2.0).sin() * (sprite_h * 0.06)
                    };
                    let dest_y = draw_start_y + bob;

                    // seleccionar textura según tipo de celda:
                    let tex = if cell == 2 {
                        coin_tex
                    } else if cell == 4 {
                        gato_tex
                    } else {
                        exit_tex
                    };

                    let dest_size = Some(vec2(sprite_w, sprite_h));
                    let params = DrawTextureParams {
                        dest_size,
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    };
                    draw_texture_ex(tex, draw_start_x, dest_y, WHITE, params);

                    // **Se elimina el brillo/ruedita blanca** que antes aparecía sobre la moneda 2.
                    // (no hay draw_circle aquí)
                }
            }
        }
    }
}


fn draw_minimap(cam: &Camera, coins: i32) {
    let ox = 10.0;
    let oy = 60.0;

    draw_rectangle(ox - 2.0, oy - 2.0, MAP_W as f32 * MINIMAP_SCALE + 4.0, MAP_H as f32 * MINIMAP_SCALE + 4.0, Color { r: 0.05, g: 0.05, b: 0.05, a: 0.8 });

    unsafe {
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                let cell = MAP[y][x];
                // Mostrar la salida (3) como muro (rojo) solo si quedan monedas; si no, mostrarla como vacía.
                let color = if is_wall(cell) {
                    GREEN
                } else if cell == 3 {
                    if coins > 0 { RED } else { BLACK }
                } else if cell == 2 || cell == 4 {
                    YELLOW
                } else {
                    BLACK
                };
                draw_rectangle(ox + x as f32 * MINIMAP_SCALE, oy + y as f32 * MINIMAP_SCALE, MINIMAP_SCALE, MINIMAP_SCALE, color);
            }
        }
    }

    let px = ox + cam.pos.x * MINIMAP_SCALE;
    let py = oy + cam.pos.y * MINIMAP_SCALE;
    draw_circle(px, py, MINIMAP_SCALE * 0.35, BLUE);

    let lx = px + cam.dir.x * MINIMAP_SCALE * 1.2;
    let ly = py + cam.dir.y * MINIMAP_SCALE * 1.2;
    draw_line(px, py, lx, ly, 1.5, WHITE);
}
