use super::gpu::{Gpu};
use crate::boxed;

pub struct GpuMode {
    pub _mode: u16,
    pub _cycles: u16,
    pub _exitFn: Box<dyn Fn(&mut Gpu) -> u16>,
}

impl GpuMode {
    pub fn new(mode: u16, cycles: u16, exitFn: impl Fn(&mut Gpu) -> u16 + 'static) -> GpuMode {
        return GpuMode {
            _cycles: cycles,
            _mode: mode,
            _exitFn: boxed!(exitFn),
        };
    }
    
    pub fn enter(&self, gpu: &mut Gpu) {
        gpu.cyclesLeft = self._cycles;
        gpu.mode = self._mode;
    }
    
    pub fn exit(&self, gpu: &mut Gpu) -> u16 {
        return (self._exitFn)(gpu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enter_updatesGpuState() {
        let mode = GpuMode::new(0, 144, |gpu: &mut Gpu| -> u16 { 0 });
        let mut gpu = Gpu::new();
        
        mode.enter(&mut gpu);
        
        assert_eq!(gpu.mode, mode._mode);
        assert_eq!(gpu.cyclesLeft, mode._cycles);
    }
    
    #[test]
    fn test_exit_returnsNextMode() {
        let mode = GpuMode::new(0, 144, |gpu: &mut Gpu| -> u16 { 1 });
        let mut gpu = Gpu::new();
        
        let result = mode.exit(&mut gpu);
        
        assert_eq!(result, 1);
    }
}
