use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use std::time::Instant;
use crate::world::{World, TilePosition};
use crate::tile::{Tile, StaticTile, LiveTile, LiveTileData};
use crate::live_tiles::{SandTile, WaterTile, ParticleTile};
use rand::{thread_rng, Rng};
use lazy_static::lazy_static;

mod world;
mod live_tile_instrution;
mod live_tile_api;
mod live_tiles;
mod tile;

type Color = [u8; 3];

const BACKGROUND_COLOR: Color = [234, 231, 217];

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    
    let world_width = 400;
    let world_height = 300;
    
    let (window, surface, mut physical_width, mut physical_height, mut scale_factor) =
        create_window("pixels test", &event_loop, world_width, world_height);

    let surface_texture = SurfaceTexture::new(physical_width, physical_height, surface);

    let mut pixels = Pixels::new(world_width, world_height, surface_texture)?;
    let mut world = World::new(BACKGROUND_COLOR, world_width, world_height);
    let mut current_frame: u64 = 0;
    let mut last_time_updated = Instant::now();
    
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            frame.copy_from_slice(&world.frame);
            pixels.render();
        }

        if input.update(event) {
            current_frame += 1;

            let current_time = Instant::now();
            let frame_time = current_time.duration_since(last_time_updated).as_nanos();
            last_time_updated = current_time;
    
            println!("{:?}", 1000000000/frame_time);

            world.update(current_frame);
            
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            let mouse_position =  input
                .mouse()
                .map(|(mx, my)| {
                    let dpx = scale_factor as f32;
                    let (w, h) = (physical_width as f32 / dpx, physical_height as f32 / dpx);
                    let mx_i = ((mx / w) * (world_width as f32)).round() as isize;
                    let my_i = ((my / h) * (world_height as f32)).round() as isize;

                    TilePosition { x: mx_i as u32, y: my_i as u32 }
                });

            if input.mouse_held(1) {
                if let Option::Some(mouse_position) = mouse_position {
                    for x in 0..3 {
                        for y in 0..3 {
                            let nx = mouse_position.x + x - 1;
                            let ny = mouse_position.y + y - 1;

                            world.set_tile(TilePosition::new(nx, ny), Tile::LiveTile(LiveTile::new(LiveTileData::Sand(SandTile::new()))));
                        }
                    }
                }
            } else if input.mouse_held(0) {
                if let Option::Some(mouse_position) = mouse_position {
                    for x in 0..3 {
                        for y in 0..3 {
                            let nx = mouse_position.x + x - 1;
                            let ny = mouse_position.y + y - 1;

                            world.set_tile(TilePosition::new(nx, ny), Tile::StaticTile(StaticTile::new([48, 47, 43])));
                        }
                    }
                }
            } else if input.mouse_held(2) {
                if let Option::Some(mouse_position) = mouse_position {
                    for x in 0..3 {
                        for y in 0..3 {
                            let nx = mouse_position.x + x - 1;
                            let ny = mouse_position.y + y - 1;
                            
                            lazy_static! {
                                static ref a: Tile = Tile::LiveTile(LiveTile::new(LiveTileData::Water(WaterTile::new())));
                            }

                            world.set_tile(
                                TilePosition::new(nx, ny), 
                                Tile::LiveTile(
                                    LiveTile::new(
                                        LiveTileData::Particle(ParticleTile::new(
                                            &a,
                                            (thread_rng().gen_range(-1.0, 1.0), thread_rng().gen_range(-1.0, 1.0))
                                        ))
                                    )
                                )
                            );

                            // world.set_tile(TilePosition::new(nx, ny), Tile::LiveTile(LiveTile::new(LiveTileData::Water(WaterTile::new()))));
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
) -> (winit::window::Window, pixels::wgpu::Surface, u32, u32, f64) {
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

    let surface = pixels::wgpu::Surface::create(&window);
    let size: PhysicalSize<f64> = default_size.to_physical(scale_factor);

    (
        window,
        surface,
        size.width.round() as u32,
        size.height.round() as u32,
        scale_factor,
    )
}
