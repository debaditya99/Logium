#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dsp;

use std::sync::{Arc, Mutex};
use tauri::{Manager, PhysicalPosition};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use tauri::menu::{Menu, MenuItem}; // NEW: Menu Imports
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleRate, StreamConfig};
use dsp::{AudioEnvironmentClassifier, BlueVoiceConfig, BlueVoicePipeline, EnvironmentMode};

struct AppState {
    pub is_auto_mode: Arc<Mutex<bool>>,
    pub current_environment: Arc<Mutex<String>>,
    pub dsp_pipeline: Arc<Mutex<BlueVoicePipeline>>,
    pub is_monitor_enabled: Arc<Mutex<bool>>,
    pub is_mic_muted: Arc<Mutex<bool>>, // NEW: Mute State
}

#[tauri::command]
fn get_live_status(state: tauri::State<AppState>) -> serde_json::Value {
    let auto = *state.is_auto_mode.lock().unwrap();
    let env = state.current_environment.lock().unwrap().clone();
    let monitor = *state.is_monitor_enabled.lock().unwrap();
    let muted = *state.is_mic_muted.lock().unwrap();
    
    serde_json::json!({
        "auto_mode": auto,
        "environment": env,
        "monitor_enabled": monitor,
        "mic_muted": muted
    })
}

#[tauri::command]
fn toggle_auto_mode(state: tauri::State<AppState>, enable: bool) {
    *state.is_auto_mode.lock().unwrap() = enable;
}

#[tauri::command]
fn toggle_monitor(state: tauri::State<AppState>, enable: bool) {
    *state.is_monitor_enabled.lock().unwrap() = enable;
}

#[tauri::command]
fn toggle_mute(state: tauri::State<AppState>, mute: bool) {
    *state.is_mic_muted.lock().unwrap() = mute;
}

// NEW: Command to let the frontend kill the app
#[tauri::command]
fn quit_app() {
    std::process::exit(0);
}

#[tauri::command]
fn set_manual_preset(state: tauri::State<AppState>, scene: String) -> serde_json::Value {
    let mode = match scene.as_str() {
        "VoiceFocused" => EnvironmentMode::VoiceFocused,
        "NoisyEnvironment" => EnvironmentMode::NoisyEnvironment,
        "QuietStudio" => EnvironmentMode::QuietStudio,
        "LateNight" => EnvironmentMode::LateNight,
        "PodcastPro" => EnvironmentMode::PodcastPro,
        "RawBypass" => EnvironmentMode::RawBypass,
        _ => EnvironmentMode::QuietStudio,
    };
    
    let new_cfg = BlueVoiceConfig::for_mode(mode);
    state.dsp_pipeline.lock().unwrap().update_config(new_cfg.clone());
    *state.current_environment.lock().unwrap() = scene.clone();

    serde_json::json!({
        "input_gain": new_cfg.input_gain * 100.0,
        "high_pass_hz": new_cfg.high_pass_hz,
        "eq_low_db": new_cfg.eq_low_db,
        "eq_mid_db": new_cfg.eq_mid_db,
        "eq_high_db": new_cfg.eq_high_db,
        "noise_reduction_db": new_cfg.noise_reduction_amount_db,
        "gate_threshold_db": new_cfg.gate_threshold_db,
        "compressor_threshold_db": new_cfg.compressor_threshold_db,
        "limiter_threshold_db": new_cfg.limiter_threshold_db,
    })
}

#[tauri::command]
fn update_dsp_param(state: tauri::State<AppState>, param: String, value: f32) {
    let mut pipe = state.dsp_pipeline.lock().unwrap();
    let mut cfg = pipe.config.clone();

    match param.as_str() {
        "input_gain" => cfg.input_gain = value / 100.0,
        "high_pass_hz" => cfg.high_pass_hz = value,
        "eq_low_db" => cfg.eq_low_db = value,
        "eq_mid_db" => cfg.eq_mid_db = value,
        "eq_high_db" => cfg.eq_high_db = value,
        "noise_reduction_db" => cfg.noise_reduction_amount_db = value,
        "gate_threshold_db" => cfg.gate_threshold_db = value,
        "compressor_threshold_db" => cfg.compressor_threshold_db = value,
        "limiter_threshold_db" => cfg.limiter_threshold_db = value,
        _ => {}
    }
    
    pipe.update_config(cfg);
}

fn main() {
    let sample_rate = 48000;
    let initial_config = BlueVoiceConfig::for_mode(EnvironmentMode::QuietStudio);
    let pipeline = Arc::new(Mutex::new(BlueVoicePipeline::new(sample_rate, initial_config)));

    let app_state = AppState {
        is_auto_mode: Arc::new(Mutex::new(true)),
        current_environment: Arc::new(Mutex::new("QuietStudio".to_string())),
        dsp_pipeline: Arc::clone(&pipeline),
        is_monitor_enabled: Arc::new(Mutex::new(false)),
        is_mic_muted: Arc::new(Mutex::new(false)), // Default to unmuted
    };

    let thread_auto_mode = Arc::clone(&app_state.is_auto_mode);
    let thread_env = Arc::clone(&app_state.current_environment);
    let thread_pipeline = Arc::clone(&pipeline);
    let thread_monitor = Arc::clone(&app_state.is_monitor_enabled);
    let thread_muted = Arc::clone(&app_state.is_mic_muted); // Clone for audio thread

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_live_status, 
            toggle_auto_mode, 
            update_dsp_param, 
            toggle_monitor, 
            set_manual_preset,
            toggle_mute,
            quit_app // NEW
        ])
        .setup(move |app| {
            // NEW: Build the Right-Click Context Menu
            let quit_i = MenuItem::with_id(app, "quit", "Quit Logium", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            TrayIconBuilder::new()
                .tooltip("Logium")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu) // Attach the menu
                .on_menu_event(|_app, event| {
                    if event.id.as_ref() == "quit" {
                        std::process::exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { position, button, button_state, .. } = event {
                        if button == MouseButton::Left && button_state == MouseButtonState::Up {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap() {
                                    window.hide().unwrap();
                                } else {
                                    let window_size = window.outer_size().unwrap();
                                    let x = (position.x as i32) - (window_size.width as i32 / 2);
                                    let y = (position.y as i32) - (window_size.height as i32) - 12;
                                    window.set_position(PhysicalPosition::new(x, y)).unwrap();
                                    window.show().unwrap();
                                    window.set_focus().unwrap();
                                }
                            }
                        }
                    }
                })
                .build(app)?;

            std::thread::spawn(move || {
                let host = cpal::default_host();
                
                let input_device = host.input_devices().unwrap()
                    .find(|d| d.name().unwrap_or_default().to_lowercase().contains("orb") || d.name().unwrap_or_default().to_lowercase().contains("yeti"))
                    .unwrap_or_else(|| host.default_input_device().expect("No default input device found"));

                let output_device = host.default_output_device().expect("No default output device found");

                let config = StreamConfig { channels: 1, sample_rate: SampleRate(48000), buffer_size: cpal::BufferSize::Default };
                let classifier = Arc::new(Mutex::new(AudioEnvironmentClassifier::new(48000)));
                let (tx, rx) = std::sync::mpsc::sync_channel::<f32>(8192);
                let class_capture = Arc::clone(&classifier);

                let input_stream = input_device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &_| {
                        let mut pipe = thread_pipeline.lock().unwrap();
                        let mut class = class_capture.lock().unwrap();
                        let is_auto = *thread_auto_mode.lock().unwrap();
                        let is_muted = *thread_muted.lock().unwrap();

                        for &sample in data {
                            if is_muted {
                                let _ = tx.try_send(0.0);
                                continue; 
                            }
                            
                            if is_auto {
                                if let Some(new_mode) = class.feed(sample) {
                                    pipe.update_config(BlueVoiceConfig::for_mode(new_mode));
                                    let mode_str = match new_mode {
                                        EnvironmentMode::VoiceFocused => "VoiceFocused",
                                        EnvironmentMode::NoisyEnvironment => "NoisyEnvironment",
                                        EnvironmentMode::QuietStudio => "QuietStudio",
                                        EnvironmentMode::LateNight => "LateNight",
                                        _ => "QuietStudio",
                                    };
                                    *thread_env.lock().unwrap() = mode_str.to_string();
                                }
                            }
                            
                            let processed = pipe.process_sample(sample);
                            let _ = tx.try_send(processed);
                        }
                    },
                    |err| eprintln!("Input stream error: {}", err),
                    None,
                ).unwrap();

                let output_stream = output_device.build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &_| {
                        // Check if live monitoring is enabled
                        let is_monitoring = *thread_monitor.lock().unwrap();
                        
                        for sample in data.iter_mut() {
                            // We MUST try_recv to clear the buffer, even if muted
                            let processed = rx.try_recv().unwrap_or(0.0);
                            
                            // If monitoring is on, play it. If not, output silence.
                            *sample = if is_monitoring { processed } else { 0.0 };
                        }
                    },
                    |err| eprintln!("Output stream error: {}", err),
                    None,
                ).unwrap();

                input_stream.play().unwrap();
                output_stream.play().unwrap();

                loop { std::thread::sleep(std::time::Duration::from_secs(1)); }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}