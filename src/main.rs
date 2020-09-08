#![windows_subsystem = "windows"]

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use std::time::Instant;
use crate::world::{World};
use crate::tile::{Tile, StaticTile, LiveTile, LiveTileState};
use crate::live_tiles::{SandTile, WaterTile};
use crate::gui::Gui;
use crate::particle::Particle;
use cgmath::Vector2;
use rand::{thread_rng, Rng};
use palette::rgb::Rgb;
use log::error;

mod world;
mod live_tiles;
mod tile;
mod particle;
mod gui;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb_color: Rgb) -> Color {
        let (r, g, b) = rgb_color.into();
        let r: u8 = (r * 255.0) as u8;
        let g: u8 = (g * 255.0) as u8;
        let b: u8 = (b * 255.0) as u8;

        Color::new(r, g, b)
    }
}

impl From<(u8, u8, u8)> for Color {    
    fn from(rgb_color: (u8, u8, u8)) -> Color {
        let (r, g, b) = rgb_color;
        Color::new(r, g, b)
    }
}

const BACKGROUND_COLOR: Color = Color { r: 234, g: 231, b: 217 };

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    
    let world_width = 400;
    let world_height = 300;
    
    let (window, mut physical_width, mut physical_height, mut scale_factor) =
        create_window("Powpowder", &event_loop, world_width, world_height);


    let mut pixels = { 
        let surface_texture = SurfaceTexture::new(physical_width, physical_height, &window);
        Pixels::new(world_width, world_height, surface_texture)? 
    };
    
    let mut world = World::new(BACKGROUND_COLOR, world_width, world_height);
    let mut current_frame: u64 = 0;
    let mut last_time_updated = Instant::now();
    
    let mut gui = Gui::new(&window, &pixels);

    #[derive(PartialEq)]
    enum SelectedItem {
        Stone,
        Sand,
        Water
    }
    
    let mut selected_item: SelectedItem = SelectedItem::Stone;
    
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            
            world.render(pixels.get_frame());

            gui.prepare(&window).expect("gui.prepare() failed");

            // pixels.render();

            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render the world texture
                context.scaling_renderer.render(encoder, render_target);

                // Render Dear ImGui
                gui.render(&window, encoder, render_target, context)
                    .expect("gui.render() failed");
            });

            if render_result
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        gui.platform
            .handle_event(gui.imgui.io_mut(), &window, &event);

        if input.update(event) {
            current_frame += 1;

            let current_time = Instant::now();
            let delta_time = current_time.duration_since(last_time_updated).as_secs_f32();
            last_time_updated = current_time;
            
            println!("FPS: {:?}", 1.0/ delta_time);

            world.update(delta_time, current_frame);
            
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            
            if input.key_pressed(VirtualKeyCode::Key1) {
                selected_item = SelectedItem::Stone
            } else if input.key_pressed(VirtualKeyCode::Key2) {
                selected_item = SelectedItem::Sand
            } else if input.key_pressed(VirtualKeyCode::Key3) {
                selected_item = SelectedItem::Water
            }

            let mouse_position =  input
                .mouse()
                .and_then(|(mx, my)| {
                    let dpx = scale_factor as f32;
                    let (w, h) = (physical_width as f32 / dpx, physical_height as f32 / dpx);
                    let mx_i = ((mx / w) * (world_width as f32)).round() as isize;
                    let my_i = ((my / h) * (world_height as f32)).round() as isize;

                    let mouse_position: Option<Vector2<u32>> = Vector2::new(mx_i, my_i).cast();
                    mouse_position
                });

            if input.mouse_held(0) && selected_item == SelectedItem::Sand {
                if let Option::Some(mouse_position) = mouse_position {
                    for _ in 0..10 {
                        world.add_particle(
                            Particle::new(
                                Tile::LiveTile(
                                    LiveTile::new(
                                        LiveTileState::Sand(SandTile::new())
                                    )
                                ),
                                mouse_position.cast().unwrap(),
                                Vector2::new(thread_rng().gen_range(-30.0, 30.0), thread_rng().gen_range(-30.0, 30.0))
                            )
                        );
                    }
                }
            } else if input.mouse_held(0) && selected_item == SelectedItem::Water {
                if let Option::Some(mouse_position) = mouse_position {
                    for _ in 0..10 {
                        world.add_particle(
                            Particle::new(
                                Tile::LiveTile(
                                    LiveTile::new(
                                        LiveTileState::Water(WaterTile::new())
                                    )
                                ),
                                mouse_position.cast().unwrap(),
                                Vector2::new(thread_rng().gen_range(-30.0, 30.0), thread_rng().gen_range(-30.0, 30.0))
                            )
                        );
                    }
                }
            } else if input.mouse_held(0) && selected_item == SelectedItem::Stone {
                if let Option::Some(mouse_position) = mouse_position {
                    for x in 0..3 {
                        for y in 0..3 {
                            let nx = mouse_position.x + x - 1;
                            let ny = mouse_position.y + y - 1;

                            if nx >= world.world_width || ny >= world.world_height {
                                continue;
                            }

                            world.set_tile(Vector2::new(nx, ny), Tile::StaticTile(StaticTile::new((48, 47, 43).into())));
                        }
                    }
                }
            }
            
            if input.mouse_held(1) {
                if let Option::Some(mouse_position) = mouse_position {
                    for x in 0..5 {
                        for y in 0..5 {
                            let nx = mouse_position.x + x - 2;
                            let ny = mouse_position.y + y - 2;

                            if nx >= world.world_width || ny >= world.world_height {
                                continue;
                            }

                            world.set_tile(Vector2::new(nx, ny), Tile::Empty);
                        }
                    }
                }
            }

            if let Some(factor) = input.scale_factor_changed() {
                scale_factor = factor;
            }

            if let Some(size) = input.window_resized() {
                physical_width = size.width;
                physical_height = size.height;
                pixels.resize(physical_width, physical_height);
            }

            window.request_redraw();
        } else {}
    });
}

fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
    world_width: u32,
    world_height: u32
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let scale_factor = window.scale_factor();

    // Get dimensions
    let width = world_width as f64;
    let height = world_height as f64;
    let (monitor_width, monitor_height) = {
        let size = window.current_monitor().size();
        (
            (size.width as f64) / scale_factor,
            (size.height as f64) / scale_factor,
        )
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round();

    // Resize, center, and display the window
    let min_size: LogicalSize<f64> = PhysicalSize::new(width, height).to_logical(scale_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    // let surface = pixels::wgpu::Surface::create(&window);
    let size: PhysicalSize<f64> = default_size.to_physical(scale_factor);

    (
        window,
        // surface,
        size.width.round() as u32,
        size.height.round() as u32,
        scale_factor,
    )
}
