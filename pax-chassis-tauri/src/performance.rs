
use std::time::{Duration, Instant};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    frame_times: VecDeque<Duration>,
    max_samples: usize,
    last_frame_start: Option<Instant>,
}

impl PerformanceMonitor {
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(max_samples),
            max_samples,
            last_frame_start: None,
        }
    }
    
    pub fn frame_start(&mut self) {
        self.last_frame_start = Some(Instant::now());
    }
    
    pub fn frame_end(&mut self) {
        if let Some(start) = self.last_frame_start.take() {
            let frame_time = start.elapsed();
            
            if self.frame_times.len() >= self.max_samples {
                self.frame_times.pop_front();
            }
            
            self.frame_times.push_back(frame_time);
        }
    }
    
    pub fn average_frame_time(&self) -> Option<Duration> {
        if self.frame_times.is_empty() {
            return None;
        }
        
        let total: Duration = self.frame_times.iter().sum();
        Some(total / self.frame_times.len() as u32)
    }
    
    pub fn current_fps(&self) -> Option<f64> {
        self.average_frame_time()
            .map(|avg| 1.0 / avg.as_secs_f64())
    }
    
    pub fn frame_time_percentile(&self, percentile: f64) -> Option<Duration> {
        if self.frame_times.is_empty() {
            return None;
        }
        
        let mut sorted_times: Vec<Duration> = self.frame_times.iter().cloned().collect();
        sorted_times.sort();
        
        let index = ((percentile / 100.0) * (sorted_times.len() - 1) as f64) as usize;
        Some(sorted_times[index])
    }
    
    pub fn is_performance_acceptable(&self, target_fps: f64) -> bool {
        if let Some(current_fps) = self.current_fps() {
            current_fps >= target_fps * 0.9 // Allow 10% tolerance
        } else {
            true // No data yet, assume acceptable
        }
    }
    
    pub fn get_summary(&self) -> PerformanceSummary {
        PerformanceSummary {
            sample_count: self.frame_times.len(),
            average_frame_time: self.average_frame_time(),
            current_fps: self.current_fps(),
            p95_frame_time: self.frame_time_percentile(95.0),
            p99_frame_time: self.frame_time_percentile(99.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub sample_count: usize,
    pub average_frame_time: Option<Duration>,
    pub current_fps: Option<f64>,
    pub p95_frame_time: Option<Duration>,
    pub p99_frame_time: Option<Duration>,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new(100) // Keep last 100 frame samples
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new(10);
        
        for _ in 0..5 {
            monitor.frame_start();
            thread::sleep(Duration::from_millis(16)); // ~60 FPS
            monitor.frame_end();
        }
        
        let summary = monitor.get_summary();
        assert_eq!(summary.sample_count, 5);
        assert!(summary.current_fps.is_some());
        assert!(summary.average_frame_time.is_some());
    }
    
    #[test]
    fn test_fps_calculation() {
        let mut monitor = PerformanceMonitor::new(10);
        
        monitor.frame_start();
        thread::sleep(Duration::from_millis(16));
        monitor.frame_end();
        
        if let Some(fps) = monitor.current_fps() {
            assert!(fps > 50.0 && fps < 70.0); // Should be around 60 FPS
        }
    }
}
