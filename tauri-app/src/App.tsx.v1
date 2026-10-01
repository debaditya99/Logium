import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { motion } from 'framer-motion';
import { Mic, Settings2, Activity, SlidersHorizontal, AudioWaveform, Headphones } from 'lucide-react';

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

export default function LogiumWidget() {
  const [status, setStatus] = useState({
    auto_mode: true,
    environment: 'QuietStudio',
    preset: 'Broadcaster',
    monitor_enabled: false,
  });

  const [config, setConfig] = useState({
    input_gain: 100,
    high_pass_hz: 77,
    eq_low_db: 1.5,
    eq_mid_db: -3.0,
    eq_high_db: 4.0,
    noise_reduction_db: 15,
    gate_threshold_db: -60,
    compressor_threshold_db: -10,
    limiter_threshold_db: 3,
  });

  const [activeTab, setActiveTab] = useState<'EQ' | 'CLEANUP'>('EQ');

  useEffect(() => {
    const interval = setInterval(async () => {
      const liveStatus: any = await invoke('get_live_status');
      setStatus(s => ({ ...s, auto_mode: liveStatus.auto_mode, environment: liveStatus.environment, monitor_enabled: liveStatus.monitor_enabled }));
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

  const updateConfig = (key: string, value: number) => {
    setConfig(prev => ({ ...prev, [key]: value }));
    invoke('update_dsp_param', { param: key, value });
  };

  return (
    <div className="h-screen w-screen bg-transparent flex flex-col justify-end p-2 pb-4">
      
      <motion.div 
        initial={false}
        animate={{ height: status.auto_mode ? 360 : 620 }}
        transition={{ type: "spring", bounce: 0, duration: 0.4 }}
        className="bg-zinc-950 text-zinc-200 p-4 border border-zinc-800 flex flex-col rounded-xl font-sans select-none overflow-hidden shadow-2xl"
      >
        
        {/* 1. Header with New Headphones Monitor Toggle */}
        <div className="flex items-center justify-between mb-4 pb-4 border-b border-zinc-800/80 shrink-0">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-full bg-indigo-500/20 flex items-center justify-center">
              <Mic size={16} className="text-indigo-400" />
            </div>
            <h1 className="text-lg font-black tracking-tight text-white uppercase">Logium</h1>
          </div>
          
          <div className="flex items-center gap-1">
            <button 
              onClick={toggleMonitor}
              title="Toggle Live Monitoring"
              className={`p-2 rounded-lg transition-colors ${status.monitor_enabled ? 'text-indigo-400 bg-indigo-500/10' : 'text-zinc-600 hover:text-zinc-400'}`}
            >
              <Headphones size={18} />
            </button>
            <button className="p-2 text-zinc-500 hover:text-white transition-colors">
              <Settings2 size={18} />
            </button>
          </div>
        </div>

        {/* 2. Dynamic Content Area */}
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
              
              <div className="flex gap-2 mb-5 shrink-0">
                <button onClick={() => setActiveTab('EQ')} className={`flex-1 py-2 text-[11px] font-bold uppercase tracking-widest rounded-lg flex items-center justify-center gap-2 transition-all ${activeTab === 'EQ' ? 'bg-zinc-800 text-white' : 'text-zinc-500 hover:bg-zinc-900'}`}>
                  <AudioWaveform size={14} /> Equalizer
                </button>
                <button onClick={() => setActiveTab('CLEANUP')} className={`flex-1 py-2 text-[11px] font-bold uppercase tracking-widest rounded-lg flex items-center justify-center gap-2 transition-all ${activeTab === 'CLEANUP' ? 'bg-zinc-800 text-white' : 'text-zinc-500 hover:bg-zinc-900'}`}>
                  <SlidersHorizontal size={14} /> Cleanup
                </button>
              </div>

              {activeTab === 'EQ' ? (
                <motion.div initial={{ opacity: 0, x: -10 }} animate={{ opacity: 1, x: 0 }} className="flex-1">
                  <Slider label="Input Gain" value={config.input_gain} min={0} max={100} step={1} unit="%" onChange={(v: number) => updateConfig('input_gain', v)} />
                  <Slider label="High-Pass Filter" value={config.high_pass_hz} min={20} max={400} step={1} unit="Hz" onChange={(v: number) => updateConfig('high_pass_hz', v)} />
                  <div className="h-px w-full bg-zinc-800/50 my-3" />
                  <Slider label="Voice EQ (Low)" value={config.eq_low_db} min={-12} max={12} step={0.5} unit="dB" onChange={(v: number) => updateConfig('eq_low_db', v)} />
                  <Slider label="Voice EQ (Mid)" value={config.eq_mid_db} min={-12} max={12} step={0.5} unit="dB" onChange={(v: number) => updateConfig('eq_mid_db', v)} />
                  <Slider label="Voice EQ (High)" value={config.eq_high_db} min={-12} max={12} step={0.5} unit="dB" onChange={(v: number) => updateConfig('eq_high_db', v)} />
                </motion.div>
              ) : (
                <motion.div initial={{ opacity: 0, x: 10 }} animate={{ opacity: 1, x: 0 }} className="flex-1 mt-2">
                  <Slider label="Noise Reduction" value={config.noise_reduction_db} min={0} max={40} step={1} unit="dB" onChange={(v: number) => updateConfig('noise_reduction_db', v)} />
                  <Slider label="Noise Gate" value={config.gate_threshold_db} min={-80} max={-6} step={1} unit="dB" onChange={(v: number) => updateConfig('gate_threshold_db', v)} />
                  <Slider label="Compressor" value={config.compressor_threshold_db} min={-40} max={0} step={1} unit="dB" onChange={(v: number) => updateConfig('compressor_threshold_db', v)} />
                  <Slider label="Limiter Ceiling" value={config.limiter_threshold_db} min={-20} max={20} step={0.5} unit="dB" onChange={(v: number) => updateConfig('limiter_threshold_db', v)} />
                </motion.div>
              )}
            </motion.div>
          )}
        </div>

        {/* 3. Auto Mode Toggle */}
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