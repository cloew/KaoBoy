use super::gpu_mode::{GpuMode};
use crate::rc_refcell;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

const LAST_SCAN_LINE: u16 = 143;
const LAST_VBLANK_LINE: u16 = 153;

const HBLANK_MODE: u16 = 0;
const VBLANK_MODE: u16 = 1;
const OAM_MODE: u16 = 2;
const VRAM_MODE: u16 = 3;

const HBLANK_CYCLES: u16 = 206; // Maybe 204 cycles
const VBLANK_CYCLES: u16 = 456;
const OAM_CYCLES: u16 = 80;
const VRAM_CYCLES: u16 = 170; // Maybe 172 cycles

pub struct Gpu {
    pub mode: u16,
    pub cyclesLeft: u16,
    pub currentLine: u16,
    
    // Modes
    oamReadMode: Rc<RefCell<GpuMode>>,
    vramReadMode: Rc<RefCell<GpuMode>>,
    hBlankMode: Rc<RefCell<GpuMode>>,
    vBlankMode: Rc<RefCell<GpuMode>>,
}

impl Gpu {
    pub fn new() -> Gpu {
        return Gpu {
            mode: 0,
            cyclesLeft: 0,
            currentLine: 0,
            // Modes
            oamReadMode: rc_refcell!(GpuMode::new(OAM_MODE, OAM_CYCLES, |gpu: &mut Gpu| -> u16 {
                return VRAM_MODE;
            })),
            vramReadMode: rc_refcell!(GpuMode::new(VRAM_MODE, VRAM_CYCLES, |gpu: &mut Gpu| -> u16 {
                // Render scanline
                return HBLANK_MODE;
            })),
            hBlankMode: rc_refcell!(GpuMode::new(HBLANK_MODE, HBLANK_CYCLES, |gpu: &mut Gpu| -> u16 {
                gpu.goToNextLine();
                if (gpu.currentLine == LAST_SCAN_LINE + 1) {
                    // Render screen
                    return VBLANK_MODE;
                } else {
                    return OAM_MODE;
                }
            })),
            vBlankMode: rc_refcell!(GpuMode::new(VBLANK_MODE, VBLANK_CYCLES, |gpu: &mut Gpu| -> u16 {
                gpu.goToNextLine();
                if (gpu.currentLine == LAST_VBLANK_LINE + 1) {
                    return OAM_MODE;
                } else {
                    return VBLANK_MODE;
                }
            })),
        };
    }
    
    pub fn initialize(&mut self) {
        self.currentLine = 0;
        self.enterMode(OAM_MODE);
    }
    
    pub fn tick(&mut self, cycles: u16) {
        // checkLYC
        let cyclesLeftToSpin = self.spin(cycles);
        if (self.cyclesLeft == 0) {
            let nextMode = self.exitMode();
            self.enterMode(nextMode);
            self.tick(cyclesLeftToSpin);
        }
    }
    
    pub fn goToNextLine(&mut self) {
        self.currentLine += 1;
    }
    
    fn enterMode(&mut self, mode: u16) {
        self.mode = mode;
        let mode = self.getCurrentMode();
        mode.borrow().enter(self);
    }
    
    fn exitMode(&mut self) -> u16 {
        let mode = self.getCurrentMode();
        return mode.borrow().exit(self);
    }
    
    fn getCurrentMode(&mut self) -> Rc<RefCell<GpuMode>> {
        return match self.mode {
            0 => self.hBlankMode.clone(),
            1 => self.vBlankMode.clone(),
            2 => self.oamReadMode.clone(),
            3 => self.vramReadMode.clone(),
            _ => panic!("Unknown mode: {}", self.mode),
        };
    }
    
    fn spin(&mut self, cycles: u16) -> u16 {
        if (cycles > self.cyclesLeft) {
            let remainingCycles = cycles - self.cyclesLeft;
            self.cyclesLeft = 0;
            return remainingCycles;
        } else {
            self.cyclesLeft -= cycles;
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initialize_prepsProperState() {
        let mut gpu = Gpu::new();
        
        gpu.initialize();
        
        assert_eq!(gpu.mode, OAM_MODE);
        assert_eq!(gpu.cyclesLeft, OAM_CYCLES);
    }
    
    #[test]
    fn test_tick_decreasesCycleCount() {
        const EXPECTED_REMAINING_CYCLES: u16 = 5;
        let mut gpu = Gpu::new();
        gpu.initialize();
        
        gpu.tick(OAM_CYCLES-EXPECTED_REMAINING_CYCLES);
        
        assert_eq!(gpu.mode, OAM_MODE);
        assert_eq!(gpu.cyclesLeft, EXPECTED_REMAINING_CYCLES);
    }
    
    #[test]
    fn test_tick_overSpins_decreasesCycleCountFromNextMode() {
        const EXPECTED_REMAINING_CYCLES: u16 = 5;
        const CYCLES_TO_SPIN: u16 = OAM_CYCLES + VRAM_CYCLES - EXPECTED_REMAINING_CYCLES;
        let mut gpu = Gpu::new();
        gpu.initialize();
        
        gpu.tick(CYCLES_TO_SPIN);
        
        assert_eq!(gpu.mode, VRAM_MODE);
        assert_eq!(gpu.cyclesLeft, EXPECTED_REMAINING_CYCLES);
    }
    
    #[test]
    fn test_tick_finishesOamMode_progressesToVramMode() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(OAM_MODE);
        
        gpu.tick(OAM_CYCLES);
        
        assert_eq!(gpu.mode, VRAM_MODE);
        assert_eq!(gpu.cyclesLeft, VRAM_CYCLES);
    }
    
    #[test]
    fn test_tick_finishesVramMode_progressesToHBlankMode() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(VRAM_MODE);
        
        gpu.tick(VRAM_CYCLES);
        
        assert_eq!(gpu.mode, HBLANK_MODE);
        assert_eq!(gpu.cyclesLeft, HBLANK_CYCLES);
    }
    
    #[test]
    fn test_tick_finishesHBlankMode_notAtLastScanLine_progressesToOamMode() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(HBLANK_MODE);
        gpu.currentLine = LAST_SCAN_LINE-1;
        
        gpu.tick(HBLANK_CYCLES);
        
        assert_eq!(gpu.mode, OAM_MODE);
        assert_eq!(gpu.cyclesLeft, OAM_CYCLES);
    }
    
    #[test]
    fn test_tick_finishesHBlankMode_atLastScanLine_progressesToVBlankMode() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(HBLANK_MODE);
        gpu.currentLine = LAST_SCAN_LINE;
        
        gpu.tick(HBLANK_CYCLES);
        
        assert_eq!(gpu.mode, VBLANK_MODE);
        assert_eq!(gpu.cyclesLeft, VBLANK_CYCLES);
    }
    
    #[test]
    fn test_tick_finishesHBlankMode_advancesToNextLine() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(HBLANK_MODE);
        let nextLine = gpu.currentLine + 1;
        
        gpu.tick(HBLANK_CYCLES);
        
        assert_eq!(gpu.currentLine, nextLine);
    }
    
    #[test]
    fn test_tick_finishesVBlankMode_notAtLastVBlankLine_progressesToVBlankMode() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(VBLANK_MODE);
        gpu.currentLine = LAST_VBLANK_LINE-1;
        
        gpu.tick(VBLANK_CYCLES);
        
        assert_eq!(gpu.mode, VBLANK_MODE);
        assert_eq!(gpu.cyclesLeft, VBLANK_CYCLES);
    }
    
    #[test]
    fn test_tick_finishesVBlankMode_atLastVBlankLine_progressesToOamMode() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(VBLANK_MODE);
        gpu.currentLine = LAST_VBLANK_LINE;
        
        gpu.tick(VBLANK_CYCLES);
        
        assert_eq!(gpu.mode, OAM_MODE);
        assert_eq!(gpu.cyclesLeft, OAM_CYCLES);
    }
    
    #[test]
    fn test_tick_finishesVBlankMode_advancesToNextLine() {
        let mut gpu = Gpu::new();
        gpu.initialize();
        gpu.enterMode(VBLANK_MODE);
        let nextLine = gpu.currentLine + 1;
        
        gpu.tick(VBLANK_CYCLES);
        
        assert_eq!(gpu.currentLine, nextLine);
    }
}
