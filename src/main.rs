use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use gb_emulator::device::Device;
use gb_emulator::KeypadKey;
use gb_emulator::AudioPlayer;
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread;
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

const EXITCODE_SUCCESS: i32 = 0;
const EXITCODE_CPULOADFAILS: i32 = 2;

enum GBEvent {
    KeyUp(KeypadKey),
    KeyDown(KeypadKey),
}

#[cfg(target_os = "windows")]
fn create_window_builder(romname: &str) -> winit::window::WindowBuilder {
    use winit::platform::windows::WindowBuilderExtWindows;
    return winit::window::WindowBuilder::new()
        .with_drag_and_drop(false)
        .with_title("RBoy - ".to_owned() + romname);
}

#[cfg(not(target_os = "windows"))]
fn create_window_builder(romname: &str) -> winit::window::WindowBuilder {
    return winit::window::WindowBuilder::new().with_title("RBoy - ".to_owned() + romname);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rboy <gamefile_name>");
        std::process::exit(1);
    }
    let filename = &args[1];

    let exit_status = real_main_minimal(filename);
    if exit_status != EXITCODE_SUCCESS {
        std::process::exit(exit_status);
    }
}

fn real_main_minimal(filename: &str) -> i32 {
    // Always use CGB mode, always enable audio, always scale 2
    let opt_classic = false;
    let opt_skip_checksum = false;
    let scale = 2;
    let opt_reload: Option<String> = None;
    let is_new_start = true;
    let cpu = construct_cpu(filename, opt_classic, opt_skip_checksum, opt_reload.clone());
    if cpu.is_none() {
        return EXITCODE_CPULOADFAILS;
    }
    let mut cpu = cpu.unwrap();

    // Always enable audio
    let player = CpalPlayer::get();
    let cpal_audio_stream = match player {
        Some((v, s)) => {
            cpu.enable_audio(Box::new(v) as Box<dyn AudioPlayer>, !is_new_start);
            Some(s)
        }
        None => {
            warn("Could not open audio device");
            return EXITCODE_CPULOADFAILS;
        }
    };
    let romname = cpu.romname();

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);

    let mut event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window_builder = create_window_builder(&romname);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(&event_loop);
    set_window_size(&window, scale);

    let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        gb_emulator::SCREEN_W as u32,
        gb_emulator::SCREEN_H as u32,
    )
    .unwrap();

    // no render options

    let cputhread = thread::spawn(move || run_cpu(cpu, sender2, receiver1));

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    'evloop: loop {
        let timeout = Some(std::time::Duration::ZERO);
        let status = event_loop.pump_events(timeout, |ev, elwt| {
            use winit::event::ElementState::{Pressed as pressed, Released as released};
            use winit::event::{Event, WindowEvent};
            use winit::keyboard::{Key, NamedKey};

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput {
                        event: keyevent, ..
                    } => match (keyevent.state, keyevent.logical_key.as_ref()) {
                        (pressed, Key::Named(NamedKey::Escape)) => elwt.exit(),
                        (pressed, winitkey) => {
                            if let Some(key) = winit_to_keypad(winitkey) {
                                let _ = sender1.send(GBEvent::KeyDown(key));
                            }
                        }
                        (released, winitkey) => {
                            if let Some(key) = winit_to_keypad(winitkey) {
                                let _ = sender1.send(GBEvent::KeyUp(key));
                            }
                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        if let PumpStatus::Exit(_) = status {
            break 'evloop;
        }
        match receiver2.recv() {
            Ok(data) => recalculate_screen(&display, &mut texture, &*data),
            Err(..) => break 'evloop, // Remote end has hung-up
        }
    }

    drop(cpal_audio_stream);
    drop(receiver2); // Stop CPU thread by disconnecting
    let _ = cputhread.join();

    EXITCODE_SUCCESS
}

fn winit_to_keypad(key: winit::keyboard::Key<&str>) -> Option<KeypadKey> {
    use winit::keyboard::{Key, NamedKey};
    match key {
        Key::Character("Z" | "z") => Some(KeypadKey::A),
        Key::Character("X" | "x") => Some(KeypadKey::B),
        Key::Named(NamedKey::ArrowUp) => Some(KeypadKey::Up),
        Key::Named(NamedKey::ArrowDown) => Some(KeypadKey::Down),
        Key::Named(NamedKey::ArrowLeft) => Some(KeypadKey::Left),
        Key::Named(NamedKey::ArrowRight) => Some(KeypadKey::Right),
        Key::Named(NamedKey::Space) => Some(KeypadKey::Select),
        Key::Named(NamedKey::Enter) => Some(KeypadKey::Start),
        _ => None,
    }
}

fn recalculate_screen<
    T: glium::glutin::surface::SurfaceTypeTrait + glium::glutin::surface::ResizeableSurface + 'static,
>(
    display: &glium::Display<T>,
    texture: &mut glium::texture::texture2d::Texture2d,
    datavec: &[u8],
) {
    use glium::Surface;

    let interpolation_type = glium::uniforms::MagnifySamplerFilter::Nearest;

    let rawimage2d = glium::texture::RawImage2d {
        data: std::borrow::Cow::Borrowed(datavec),
        width: gb_emulator::SCREEN_W as u32,
        height: gb_emulator::SCREEN_H as u32,
        format: glium::texture::ClientFormat::U8U8U8,
    };
    texture.write(
        glium::Rect {
            left: 0,
            bottom: 0,
            width: gb_emulator::SCREEN_W as u32,
            height: gb_emulator::SCREEN_H as u32,
        },
        rawimage2d,
    );

    // We use a custom BlitTarget to transform OpenGL coordinates to row-column coordinates
    let target = display.draw();
    let (target_w, target_h) = target.get_dimensions();
    texture.as_surface().blit_whole_color_to(
        &target,
        &glium::BlitTarget {
            left: 0,
            bottom: target_h,
            width: target_w as i32,
            height: -(target_h as i32),
        },
        interpolation_type,
    );
    target.finish().unwrap();
}

fn warn(message: &str) {
    eprintln!("{}", message);
}

fn construct_cpu(
    filename: &str,
    classic_mode: bool,
    skip_checksum: bool,
    reload_mode: Option<String>,
) -> Option<Box<Device>> {
    let opt_c = match classic_mode {
        true => Device::new(filename, skip_checksum, reload_mode),
        false => Device::new_cgb(filename, skip_checksum, reload_mode),
    };
    let c = match opt_c {
        Ok(cpu) => cpu,
        Err(message) => {
            warn(message);
            return None;
        }
    };

    Some(Box::new(c))
}

fn run_cpu(mut cpu: Box<Device>, sender: SyncSender<Vec<u8>>, receiver: Receiver<GBEvent>) {
    let periodic = timer_periodic(16);

    let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
    let mut ticks = 0;

    'outer: loop {
        while ticks < waitticks {
            ticks += cpu.do_cycle();
            if cpu.check_and_reset_gpu_updated() {
                let data = cpu.get_gpu_data().to_vec();
                if let Err(TrySendError::Disconnected(..)) = sender.try_send(data) {
                    break 'outer;
                }
            }
        }

        ticks -= waitticks;

        'recv: loop {
            match receiver.try_recv() {
                Ok(event) => match event {
                    GBEvent::KeyUp(key) => cpu.keyup(key),
                    GBEvent::KeyDown(key) => cpu.keydown(key),
                },
                Err(TryRecvError::Empty) => break 'recv,
                Err(TryRecvError::Disconnected) => break 'outer,
            }
        }

        // throttle timing each frame
        let _ = periodic.recv();
    }
}

fn timer_periodic(ms: u64) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(ms));
        if tx.send(()).is_err() {
            break;
        }
    });
    rx
}

fn set_window_size(window: &winit::window::Window, scale: u32) {
    let _ = window.request_inner_size(winit::dpi::LogicalSize::<u32>::from((
        gb_emulator::SCREEN_W as u32 * scale,
        gb_emulator::SCREEN_H as u32 * scale,
    )));
}

struct CpalPlayer {
    buffer: Arc<Mutex<Vec<(f32, f32)>>>,
    sample_rate: u32,
}

impl CpalPlayer {
    fn get() -> Option<(CpalPlayer, cpal::Stream)> {
        let device = match cpal::default_host().default_output_device() {
            Some(e) => e,
            None => return None,
        };

        // We want a config with:
        // chanels = 2
        // SampleFormat F32
        // Rate at around 44100

        let wanted_samplerate = cpal::SampleRate(44100);
        let supported_configs = match device.supported_output_configs() {
            Ok(e) => e,
            Err(_) => return None,
        };
        let mut supported_config = None;
        for f in supported_configs {
            if f.channels() == 2 && f.sample_format() == cpal::SampleFormat::F32 {
                if f.min_sample_rate() <= wanted_samplerate
                    && wanted_samplerate <= f.max_sample_rate()
                {
                    supported_config = Some(f.with_sample_rate(wanted_samplerate));
                } else {
                    supported_config = Some(f.with_max_sample_rate());
                }
                break;
            }
        }
        if supported_config.is_none() {
            return None;
        }

        let selected_config = supported_config.unwrap();

        let sample_format = selected_config.sample_format();
        let config: cpal::StreamConfig = selected_config.into();

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let shared_buffer = Arc::new(Mutex::new(Vec::new()));
        let stream_buffer = shared_buffer.clone();

        let player = CpalPlayer {
            buffer: shared_buffer,
            sample_rate: config.sample_rate.0,
        };

        let stream = match sample_format {
            cpal::SampleFormat::I8 => device.build_output_stream(
                &config,
                move |data: &mut [i8], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_output_stream(
                &config,
                move |data: &mut [i16], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I32 => device.build_output_stream(
                &config,
                move |data: &mut [i32], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I64 => device.build_output_stream(
                &config,
                move |data: &mut [i64], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U8 => device.build_output_stream(
                &config,
                move |data: &mut [u8], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_output_stream(
                &config,
                move |data: &mut [u16], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U32 => device.build_output_stream(
                &config,
                move |data: &mut [u32], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U64 => device.build_output_stream(
                &config,
                move |data: &mut [u64], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config,
                move |data: &mut [f32], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::F64 => device.build_output_stream(
                &config,
                move |data: &mut [f64], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            sf => panic!("Unsupported sample format {}", sf),
        }
        .unwrap();

        stream.play().unwrap();

        Some((player, stream))
    }
}

fn cpal_thread<T: Sample + FromSample<f32>>(
    outbuffer: &mut [T],
    audio_buffer: &Arc<Mutex<Vec<(f32, f32)>>>,
) {
    let mut inbuffer = audio_buffer.lock().unwrap();
    let outlen = ::std::cmp::min(outbuffer.len() / 2, inbuffer.len());
    for (i, (in_l, in_r)) in inbuffer.drain(..outlen).enumerate() {
        outbuffer[i * 2] = T::from_sample(in_l);
        outbuffer[i * 2 + 1] = T::from_sample(in_r);
    }
}

impl AudioPlayer for CpalPlayer {
    fn play(&mut self, buf_left: &[f32], buf_right: &[f32]) {
        debug_assert!(buf_left.len() == buf_right.len());

        let mut buffer = self.buffer.lock().unwrap();

        for (l, r) in buf_left.iter().zip(buf_right) {
            if buffer.len() > self.sample_rate as usize {
                // Do not fill the buffer with more than 1 second of data
                // This speeds up the resync after the turning on and off the speed limiter
                return;
            }
            buffer.push((*l, *r));
        }
    }

    fn samples_rate(&self) -> u32 {
        self.sample_rate
    }

    fn underflowed(&self) -> bool {
        (*self.buffer.lock().unwrap()).len() == 0
    }
}
