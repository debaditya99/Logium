import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { motion, AnimatePresence } from 'framer-motion';
import { Mic, MicOff, Settings2, Activity, SlidersHorizontal, AudioWaveform, Headphones, Layers, Moon, Radio, ShieldAlert, Coffee, ZapOff, RotateCcw, Power } from 'lucide-react';

const Slider = ({ label, value, min, max, step, unit, onChange }: any) => (
  <div className="mb-3">
    <div className="flex justify-between items-end mb-1.5">
      <label className="text-[10px] font-bold tracking-widest text-zinc-400 uppercase">{label}</label>
      <span className="text-xs font-bold text-white bg-zinc-800/80 px-2 py-0.5 rounded">{value}{unit}</span>
    </div>
    <input
      type="range"
      min={min} max={max} step={step}
      value={value}
      onChange={(e) => onChange(parseFloat(e.target.value))}
      className="w-full h-1.5 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-indigo-500 hover:accent-indigo-400 transition-all"
    />
  </div>
);

const PRESETS = [
  { id: 'QuietStudio', icon: Coffee, name: 'Quiet Studio', desc: 'Natural & relaxed' },
  { id: 'VoiceFocused', icon: Mic, name: 'Voice Focused', desc: 'Clear & present' },
  { id: 'NoisyEnvironment', icon: ShieldAlert, name: 'Noisy Area', desc: 'Aggressive gating' },
  { id: 'LateNight', icon: Moon, name: 'Late Night', desc: 'Whisper optimized' },
  { id: 'PodcastPro', icon: Radio, name: 'Podcast Pro', desc: 'Deep broadcast tone' },
  { id: 'RawBypass', icon: ZapOff, name: 'Raw Bypass', desc: 'Unprocessed mic' },
];

export default function LogiumWidget() {
  const [status, setStatus] = useState({
    auto_mode: true,
    environment: 'QuietStudio',
    monitor_enabled: false,
    mic_muted: false,
  });

  const [config, setConfig] = useState({
    input_gain: 100, high_pass_hz: 77, eq_low_db: 1.5, eq_mid_db: -3.0, eq_high_db: 4.0,
    noise_reduction_db: 15, gate_threshold_db: -60, compressor_threshold_db: -10, limiter_threshold_db: 3,
  });

  const [activeTab, setActiveTab] = useState<'PRESETS' | 'EQ' | 'CLEANUP'>('PRESETS');
  const [showSettings, setShowSettings] = useState(false); // NEW: Settings Overlay State

  useEffect(() => {
    const interval = setInterval(async () => {
      const liveStatus: any = await invoke('get_live_status');
      setStatus(s => ({ 
        ...s, 
        auto_mode: liveStatus.auto_mode, 
        environment: liveStatus.environment, 
        monitor_enabled: liveStatus.monitor_enabled,
        mic_muted: liveStatus.mic_muted 
      }));
    }, 500);
    return () => clearInterval(interval);
  }, []);

  const toggleAuto = async () => {
    const nextState = !status.auto_mode;
    await invoke('toggle_auto_mode', { enable: nextState });
    setStatus(prev => ({ ...prev, auto_mode: nextState }));
  };

  const toggleMonitor = async () => {
    const nextState = !status.monitor_enabled;
    await invoke('toggle_monitor', { enable: nextState });
    setStatus(prev => ({ ...prev, monitor_enabled: nextState }));
  };

  const toggleMute = async () => {
    const nextState = !status.mic_muted;
    await invoke('toggle_mute', { mute: nextState });
    setStatus(prev => ({ ...prev, mic_muted: nextState }));
  };

  const updateConfig = (key: string, value: number) => {
    setConfig(prev => ({ ...prev, [key]: value }));
    invoke('update_dsp_param', { param: key, value });
  };

  const applyPreset = async (sceneId: string) => {
    const newConfig: any = await invoke('set_manual_preset', { scene: sceneId });
    setConfig(newConfig);
    setStatus(prev => ({ ...prev, environment: sceneId }));
  };

  return (
    <div className="h-screen w-screen bg-transparent flex flex-col justify-end p-2 pb-4 relative">
      <motion.div 
        initial={false}
        animate={{ height: status.auto_mode ? 360 : 620 }}
        transition={{ type: "spring", bounce: 0, duration: 0.4 }}
        className="bg-zinc-950 text-zinc-200 p-4 border border-zinc-800 flex flex-col rounded-xl font-sans select-none overflow-hidden shadow-2xl relative"
      >
        
        {/* Header */}
        <div className="flex items-center justify-between mb-4 pb-4 border-b border-zinc-800/80 shrink-0">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-full bg-indigo-500/20 flex items-center justify-center">
              <Mic size={16} className="text-indigo-400" />
            </div>
            <h1 className="text-lg font-black tracking-tight text-white uppercase">Logium</h1>
          </div>
          
          <div className="flex items-center gap-1">
            <button 
              onClick={toggleMute}
              title={status.mic_muted ? "Unmute Microphone" : "Mute Microphone"}
              className={`p-2 rounded-lg transition-colors ${status.mic_muted ? 'text-red-400 bg-red-500/10' : 'text-zinc-600 hover:text-red-400'}`}
            >
              {status.mic_muted ? <MicOff size={18} /> : <Mic size={18} />}
            </button>
            <button 
              onClick={toggleMonitor}
              title="Toggle Live Monitoring"
              className={`p-2 rounded-lg transition-colors ${status.monitor_enabled ? 'text-indigo-400 bg-indigo-500/10' : 'text-zinc-600 hover:text-zinc-400'}`}
            >
              <Headphones size={18} />
            </button>
            
            {/* NEW: Settings Toggle */}
            <button 
              onClick={() => setShowSettings(!showSettings)}
              className={`p-2 rounded-lg transition-colors ${showSettings ? 'text-white bg-zinc-800' : 'text-zinc-500 hover:text-white'}`}
            >
              <Settings2 size={18} />
            </button>
          </div>
        </div>

        {/* NEW: Settings Overlay Panel */}
        <AnimatePresence>
          {showSettings && (
            <motion.div
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0, scale: 0.95 }}
              className="absolute inset-x-4 top-20 bottom-4 bg-zinc-950/95 backdrop-blur-md z-50 rounded-xl border border-zinc-800 shadow-2xl p-6 flex flex-col"
            >
              <h3 className="text-xs font-bold text-zinc-400 uppercase tracking-widest mb-6">System Preferences</h3>
              
              <div className="space-y-3">
                <button 
                  onClick={() => { applyPreset(status.environment); setShowSettings(false); }} 
                  className="w-full flex items-center justify-between px-4 py-3 bg-zinc-900 hover:bg-zinc-800 rounded-lg text-sm font-medium text-zinc-200 transition-colors border border-zinc-800/80"
                >
                  Reset Current Preset
                  <RotateCcw size={14} className="text-zinc-500" />
                </button>
                
                <button 
                  onClick={() => invoke('quit_app')} 
                  className="w-full flex items-center justify-between px-4 py-3 bg-red-500/10 hover:bg-red-500/20 text-red-400 rounded-lg text-sm font-medium transition-colors border border-red-500/20"
                >
                  Quit Logium
                  <Power size={14} />
                </button>
              </div>

              <div className="mt-auto text-center pb-2">
                <p className="text-[10px] font-bold text-zinc-600 uppercase tracking-widest">Logium v0.1.0</p>
                <p className="text-[9px] text-zinc-700 mt-1">Rust DSP Engine Active</p>
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* Content Area */}
        <div className="flex-1 flex flex-col min-h-0 mb-4">
          {status.auto_mode ? (
            <motion.div 
              initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
              className="bg-zinc-900/40 border border-indigo-500/20 p-4 rounded-xl flex flex-col items-center justify-center text-center space-y-3 h-full"
            >
              <Activity size={36} className="text-indigo-400 mb-1 opacity-80" />
              <span className="text-[10px] font-bold text-indigo-500 uppercase tracking-widest">Active Scene</span>
              <h3 className="text-2xl font-black text-white">{status.environment}</h3>
            </motion.div>
          ) : (
            <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="flex flex-col h-full">
              
              <div className="flex gap-1 mb-4 shrink-0 bg-zinc-900/50 p-1 rounded-xl border border-zinc-800/80">
                <button onClick={() => setActiveTab('PRESETS')} className={`flex-1 py-1.5 text-[9px] font-bold uppercase tracking-widest rounded-lg flex flex-col items-center justify-center gap-1 transition-all ${activeTab === 'PRESETS' ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'}`}>
                  <Layers size={14} /> Presets
                </button>
                <button onClick={() => setActiveTab('EQ')} className={`flex-1 py-1.5 text-[9px] font-bold uppercase tracking-widest rounded-lg flex flex-col items-center justify-center gap-1 transition-all ${activeTab === 'EQ' ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'}`}>
                  <AudioWaveform size={14} /> EQ
                </button>
                <button onClick={() => setActiveTab('CLEANUP')} className={`flex-1 py-1.5 text-[9px] font-bold uppercase tracking-widest rounded-lg flex flex-col items-center justify-center gap-1 transition-all ${activeTab === 'CLEANUP' ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'}`}>
                  <SlidersHorizontal size={14} /> Cleanup
                </button>
              </div>

              {activeTab === 'PRESETS' && (
                <motion.div initial={{ opacity: 0, x: -10 }} animate={{ opacity: 1, x: 0 }} className="grid grid-cols-2 gap-2 flex-1 mt-1">
                  {PRESETS.map(p => (
                    <button
                      key={p.id}
                      onClick={() => applyPreset(p.id)}
                      className={`p-3 rounded-xl border text-left flex flex-col gap-2 transition-all ${status.environment === p.id ? 'bg-indigo-500/10 border-indigo-500/50 text-white shadow-sm' : 'bg-zinc-900/50 border-zinc-800/80 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'}`}
                    >
                      <p.icon size={16} className={status.environment === p.id ? 'text-indigo-400' : 'text-zinc-500'} />
                      <div>
                        <div className="text-xs font-bold">{p.name}</div>
                        <div className="text-[9px] opacity-70 mt-0.5">{p.desc}</div>
                      </div>
                    </button>
                  ))}
                </motion.div>
              )}

              {activeTab === 'EQ' && (
                <motion.div initial={{ opacity: 0, x: -10 }} animate={{ opacity: 1, x: 0 }} className="flex-1 mt-1">
                  <Slider label="Input Gain" value={config.input_gain} min={0} max={200} step={1} unit="%" onChange={(v: number) => updateConfig('input_gain', v)} />
                  <Slider label="High-Pass Filter" value={config.high_pass_hz} min={20} max={400} step={1} unit="Hz" onChange={(v: number) => updateConfig('high_pass_hz', v)} />
                  <div className="h-px w-full bg-zinc-800/50 my-3" />
                  <Slider label="Voice EQ (Low)" value={config.eq_low_db} min={-12} max={12} step={0.5} unit="dB" onChange={(v: number) => updateConfig('eq_low_db', v)} />
                  <Slider label="Voice EQ (Mid)" value={config.eq_mid_db} min={-12} max={12} step={0.5} unit="dB" onChange={(v: number) => updateConfig('eq_mid_db', v)} />
                  <Slider label="Voice EQ (High)" value={config.eq_high_db} min={-12} max={12} step={0.5} unit="dB" onChange={(v: number) => updateConfig('eq_high_db', v)} />
                </motion.div>
              )}

              {activeTab === 'CLEANUP' && (
                <motion.div initial={{ opacity: 0, x: 10 }} animate={{ opacity: 1, x: 0 }} className="flex-1 mt-1">
                  <Slider label="Noise Reduction" value={config.noise_reduction_db} min={0} max={40} step={1} unit="dB" onChange={(v: number) => updateConfig('noise_reduction_db', v)} />
                  <Slider label="Noise Gate" value={config.gate_threshold_db} min={-80} max={-6} step={1} unit="dB" onChange={(v: number) => updateConfig('gate_threshold_db', v)} />
                  <Slider label="Compressor" value={config.compressor_threshold_db} min={-40} max={0} step={1} unit="dB" onChange={(v: number) => updateConfig('compressor_threshold_db', v)} />
                  <Slider label="Limiter Ceiling" value={config.limiter_threshold_db} min={-20} max={20} step={0.5} unit="dB" onChange={(v: number) => updateConfig('limiter_threshold_db', v)} />
                </motion.div>
              )}
            </motion.div>
          )}
        </div>

        {/* Auto Mode Toggle */}
        <div className="flex items-center justify-between bg-zinc-900/60 p-4 rounded-xl border border-zinc-800/80 shrink-0">
          <div>
            <h2 className="font-bold text-white text-sm">Smart Auto-Tuner</h2>
            <p className="text-[11px] text-zinc-500">Adapts to room noise dynamically</p>
          </div>
          <button 
            onClick={toggleAuto}
            className={`relative w-12 h-6 rounded-full transition-colors duration-300 ${status.auto_mode ? 'bg-indigo-500' : 'bg-zinc-700'}`}
          >
            <motion.div 
              layout
              className="absolute top-1 left-1 w-4 h-4 bg-white rounded-full shadow-sm"
              animate={{ x: status.auto_mode ? 24 : 0 }}
              transition={{ type: "spring", stiffness: 500, damping: 30 }}
            />
          </button>
        </div>

      </motion.div>
    </div>
  );
}