use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type, Q_BUTTERWORTH_F32};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnvironmentMode {
    VoiceFocused,
    NoisyEnvironment,
    QuietStudio,
    LateNight,   // NEW: Whisper optimized
    PodcastPro,  // NEW: Deep broadcast tone (Manual only)
    RawBypass,   // NEW: Unprocessed (Manual only)
}

#[derive(Clone, Debug)]
pub struct BlueVoiceConfig {
    pub input_gain: f32,
    pub master_output_level: f32,
    pub high_pass_enabled: bool,
    pub high_pass_hz: f32,
    pub voice_eq_enabled: bool,
    pub eq_low_hz: f32,
    pub eq_low_db: f32,
    pub eq_mid_hz: f32,
    pub eq_mid_db: f32,
    pub eq_high_hz: f32,
    pub eq_high_db: f32,
    pub noise_reduction_enabled: bool,
    pub noise_reduction_amount_db: f32,
    pub gate_enabled: bool,
    pub gate_threshold_db: f32,
    pub compressor_enabled: bool,
    pub compressor_threshold_db: f32,
    pub de_esser_enabled: bool,
    pub de_esser_threshold_db: f32,
    pub de_popper_enabled: bool,
    pub de_popper_threshold_db: f32,
    pub limiter_enabled: bool,
    pub limiter_threshold_db: f32,
}

impl Default for BlueVoiceConfig {
    fn default() -> Self {
        Self {
            input_gain: 1.0,
            master_output_level: 1.0,
            high_pass_enabled: true,
            high_pass_hz: 77.0,
            voice_eq_enabled: true,
            eq_low_hz: 150.0,
            eq_low_db: 1.5,
            eq_mid_hz: 450.0,
            eq_mid_db: -3.0,
            eq_high_hz: 6500.0,
            eq_high_db: 4.0,
            noise_reduction_enabled: true,
            noise_reduction_amount_db: 15.0,
            gate_enabled: false,
            gate_threshold_db: -60.0,
            compressor_enabled: true,
            compressor_threshold_db: -10.0,
            de_esser_enabled: true,
            de_esser_threshold_db: -15.0,
            de_popper_enabled: true,
            de_popper_threshold_db: -15.0,
            limiter_enabled: true,
            limiter_threshold_db: 3.0,
        }
    }
}
impl BlueVoiceConfig {
    pub fn for_mode(mode: EnvironmentMode) -> Self {
        let mut cfg = Self::default();
        match mode {
            EnvironmentMode::VoiceFocused => {
                cfg.high_pass_hz = 77.0;
                cfg.voice_eq_enabled = true;
                cfg.eq_low_db = 2.0;
                cfg.eq_mid_db = -1.5;
                cfg.eq_high_db = 3.5;
                cfg.noise_reduction_enabled = true;
                cfg.noise_reduction_amount_db = 10.0;
                cfg.gate_enabled = true;
                cfg.gate_threshold_db = -50.0;
                cfg.compressor_enabled = true;
                cfg.compressor_threshold_db = -12.0;
            }
            EnvironmentMode::NoisyEnvironment => {
                cfg.high_pass_hz = 125.0;
                cfg.voice_eq_enabled = true;
                cfg.eq_low_db = -2.0;
                cfg.eq_mid_db = 2.0;
                cfg.eq_high_db = 1.0;
                cfg.noise_reduction_enabled = true;
                cfg.noise_reduction_amount_db = 25.0;
                cfg.gate_enabled = true;
                cfg.gate_threshold_db = -38.0;
                cfg.compressor_enabled = true;
                cfg.compressor_threshold_db = -8.0;
            }
            EnvironmentMode::QuietStudio => {
                cfg.high_pass_hz = 60.0;
                cfg.voice_eq_enabled = true;
                cfg.eq_low_db = 1.0;
                cfg.eq_mid_db = 0.0;
                cfg.eq_high_db = 2.0;
                cfg.noise_reduction_enabled = false;
                cfg.gate_enabled = true;
                cfg.gate_threshold_db = -65.0;
                cfg.compressor_enabled = true;
                cfg.compressor_threshold_db = -15.0;
            }
            EnvironmentMode::LateNight => {
                cfg.input_gain = 1.5; // +50% gain to pick up whispers
                cfg.high_pass_hz = 60.0;
                cfg.voice_eq_enabled = true;
                cfg.eq_low_db = 2.0;
                cfg.eq_mid_db = 0.0;
                cfg.eq_high_db = 1.0;
                cfg.noise_reduction_enabled = true;
                cfg.noise_reduction_amount_db = 20.0;
                cfg.gate_enabled = true;
                cfg.gate_threshold_db = -55.0;
                cfg.compressor_enabled = true;
                cfg.compressor_threshold_db = -25.0; // Heavy compression to level out whispers
                cfg.limiter_threshold_db = 0.0;
            }
            EnvironmentMode::PodcastPro => {
                cfg.high_pass_hz = 90.0;
                cfg.voice_eq_enabled = true;
                cfg.eq_low_db = 3.5;  // Deep radio bass
                cfg.eq_mid_db = -1.0; // Scoop boxy mids
                cfg.eq_high_db = 4.5; // Crisp articulation
                cfg.noise_reduction_enabled = true;
                cfg.noise_reduction_amount_db = 15.0;
                cfg.gate_enabled = true;
                cfg.gate_threshold_db = -45.0;
                cfg.compressor_enabled = true;
                cfg.compressor_threshold_db = -18.0; 
                cfg.limiter_threshold_db = 2.0;
            }
            EnvironmentMode::RawBypass => {
                cfg.input_gain = 1.0;
                cfg.high_pass_enabled = false;
                cfg.voice_eq_enabled = false;
                cfg.noise_reduction_enabled = false;
                cfg.gate_enabled = false;
                cfg.compressor_enabled = false;
                cfg.de_esser_enabled = false;
                cfg.de_popper_enabled = false;
                cfg.limiter_enabled = false;
            }
        }
        cfg
    }
}

pub struct BlueVoicePipeline {
    sample_rate: u32,
    pub config: BlueVoiceConfig,
    hpf_filter: Option<DirectForm1<f32>>,
    low_shelf: Option<DirectForm1<f32>>,
    mid_peak: Option<DirectForm1<f32>>,
    high_shelf: Option<DirectForm1<f32>>,
    gate_envelope: f32,
    gate_gain: f32,
    de_popper_hpf: Option<DirectForm1<f32>>,
    de_popper_env: f32,
    de_esser_bandpass: Option<DirectForm1<f32>>,
    de_esser_env: f32,
    comp_envelope: f32,
}

impl BlueVoicePipeline {
    pub fn new(sample_rate: u32, config: BlueVoiceConfig) -> Self {
        let mut pipeline = Self {
            sample_rate,
            config: config.clone(),
            hpf_filter: None,
            low_shelf: None,
            mid_peak: None,
            high_shelf: None,
            gate_envelope: 0.0,
            gate_gain: 1.0,
            de_popper_hpf: None,
            de_popper_env: 0.0,
            de_esser_bandpass: None,
            de_esser_env: 0.0,
            comp_envelope: 0.0,
        };
        pipeline.rebuild_filters();
        pipeline
    }

    pub fn update_config(&mut self, config: BlueVoiceConfig) {
        self.config = config;
        self.rebuild_filters();
    }

    pub fn rebuild_filters(&mut self) {
        let sr = self.sample_rate.hz();
        if self.config.high_pass_enabled {
            let coef = Coefficients::<f32>::from_params(Type::HighPass, sr, self.config.high_pass_hz.clamp(20.0, 400.0).hz(), Q_BUTTERWORTH_F32).unwrap();
            self.hpf_filter = Some(DirectForm1::<f32>::new(coef));
        } else {
            self.hpf_filter = None;
        }

        if self.config.voice_eq_enabled {
            let low_coef = Coefficients::<f32>::from_params(Type::LowShelf(self.config.eq_low_db), sr, self.config.eq_low_hz.hz(), Q_BUTTERWORTH_F32).unwrap();
            let mid_coef = Coefficients::<f32>::from_params(Type::PeakingEQ(self.config.eq_mid_db), sr, self.config.eq_mid_hz.hz(), Q_BUTTERWORTH_F32).unwrap();
            let high_coef = Coefficients::<f32>::from_params(Type::HighShelf(self.config.eq_high_db), sr, self.config.eq_high_hz.hz(), Q_BUTTERWORTH_F32).unwrap();
            self.low_shelf = Some(DirectForm1::<f32>::new(low_coef));
            self.mid_peak = Some(DirectForm1::<f32>::new(mid_coef));
            self.high_shelf = Some(DirectForm1::<f32>::new(high_coef));
        } else {
            self.low_shelf = None;
            self.mid_peak = None;
            self.high_shelf = None;
        }

        let de_pop_coef = Coefficients::<f32>::from_params(Type::HighPass, sr, 120.0.hz(), Q_BUTTERWORTH_F32).unwrap();
        self.de_popper_hpf = Some(DirectForm1::<f32>::new(de_pop_coef));

        let de_ess_coef = Coefficients::<f32>::from_params(Type::BandPass, sr, 6500.0.hz(), 2.0).unwrap();
        self.de_esser_bandpass = Some(DirectForm1::<f32>::new(de_ess_coef));
    }

    pub fn process_sample(&mut self, mut s: f32) -> f32 {
        s *= self.config.input_gain;

        if let Some(ref mut hpf) = self.hpf_filter {
            s = hpf.run(s);
        }

        if self.config.de_popper_enabled {
            let abs_s = s.abs();
            let pop_thresh = 10.0f32.powf(self.config.de_popper_threshold_db / 20.0);
            self.de_popper_env = 0.95 * self.de_popper_env + 0.05 * abs_s;
            if self.de_popper_env > pop_thresh {
                if let Some(ref mut pop_filt) = self.de_popper_hpf { s = pop_filt.run(s); }
            }
        }

        if self.config.noise_reduction_enabled {
            let reduction_factor = 10.0f32.powf(-self.config.noise_reduction_amount_db / 40.0);
            let energy = s * s;
            if energy < 0.0005 { s *= reduction_factor; }
        }

        if self.config.gate_enabled {
            let gate_thresh = 10.0f32.powf(self.config.gate_threshold_db / 20.0);
            let abs_s = s.abs();
            if abs_s > self.gate_envelope {
                self.gate_envelope = 0.05 * abs_s + 0.95 * self.gate_envelope;
            } else {
                self.gate_envelope = 0.999 * self.gate_envelope;
            }
            let target_gain = if self.gate_envelope > gate_thresh { 1.0 } else { 0.0 };
            self.gate_gain = 0.90 * self.gate_gain + 0.10 * target_gain;
            s *= self.gate_gain;
        }

        if let (Some(low), Some(mid), Some(high)) = (&mut self.low_shelf, &mut self.mid_peak, &mut self.high_shelf) {
            s = low.run(s);
            s = mid.run(s);
            s = high.run(s);
        }

        if self.config.de_esser_enabled {
            if let Some(ref mut bp) = self.de_esser_bandpass {
                let sibilance = bp.run(s).abs();
                let ess_thresh = 10.0f32.powf(self.config.de_esser_threshold_db / 20.0);
                self.de_esser_env = 0.8 * self.de_esser_env + 0.2 * sibilance;
                if self.de_esser_env > ess_thresh {
                    let duck = (ess_thresh / self.de_esser_env).clamp(0.2, 1.0);
                    s *= duck;
                }
            }
        }

        if self.config.compressor_enabled {
            let comp_thresh = 10.0f32.powf(self.config.compressor_threshold_db / 20.0);
            let abs_s = s.abs();
            self.comp_envelope = 0.9 * self.comp_envelope + 0.1 * abs_s;
            if self.comp_envelope > comp_thresh {
                let ratio = 3.5;
                let over = self.comp_envelope - comp_thresh;
                let compressed = comp_thresh + (over / ratio);
                let gain = compressed / self.comp_envelope;
                s *= gain;
            }
        }

        s *= self.config.master_output_level;

        if self.config.limiter_enabled {
            let limit_ceil = 10.0f32.powf(self.config.limiter_threshold_db / 20.0).clamp(0.1, 1.0);
            s = s.clamp(-limit_ceil, limit_ceil);
        }

        s
    }
}

pub struct AudioEnvironmentClassifier {
    sample_rate: u32,
    window_samples: Vec<f32>,
    window_size: usize,
    pub current_mode: EnvironmentMode,
    consecutive_hits: usize,
}

impl AudioEnvironmentClassifier {
    pub fn new(sample_rate: u32) -> Self {
        let window_size = (sample_rate as f32 * 0.5) as usize;
        Self {
            sample_rate,
            window_samples: Vec::with_capacity(window_size),
            window_size,
            current_mode: EnvironmentMode::QuietStudio,
            consecutive_hits: 0,
        }
    }

    pub fn feed(&mut self, sample: f32) -> Option<EnvironmentMode> {
        self.window_samples.push(sample);

        if self.window_samples.len() >= self.window_size {
            let next_mode = self.analyze();
            self.window_samples.clear();

            if next_mode != self.current_mode {
                self.consecutive_hits += 1;
                if self.consecutive_hits >= 2 {
                    self.current_mode = next_mode;
                    self.consecutive_hits = 0;
                    return Some(next_mode);
                }
            } else {
                self.consecutive_hits = 0;
            }
        }
        None
    }

    fn analyze(&self) -> EnvironmentMode {
        let n = self.window_samples.len() as f32;
        let mut sum_sq = 0.0;
        let mut zero_crossings = 0;

        for i in 0..self.window_samples.len() {
            let s = self.window_samples[i];
            sum_sq += s * s;
            if i > 0 && ((self.window_samples[i - 1] >= 0.0 && s < 0.0) || (self.window_samples[i - 1] < 0.0 && s >= 0.0)) {
                zero_crossings += 1;
            }
        }

        let rms = (sum_sq / n).sqrt();
        let rms_db = if rms > 1e-6 { 20.0 * rms.log10() } else { -100.0 };
        let zcr = zero_crossings as f32 / n;

        if rms_db > -35.0 && zcr < 0.18 {
            EnvironmentMode::VoiceFocused
        } else if rms_db > -44.0 && zcr >= 0.12 {
            EnvironmentMode::NoisyEnvironment
        } else if rms_db <= -35.0 && rms_db > -55.0 && zcr < 0.18 {
            // Low volume but vocal ZCR = Whispering
            EnvironmentMode::LateNight
        } else {
            EnvironmentMode::QuietStudio
        }
    }
}