use std::time::{Instant, Duration};

const MEASUREMENT_INTERVAL_DURATION: Duration = Duration::from_millis(1000);

pub struct MainLoopClock {
    current_frame_start_instant: Instant,
    current_measurement_interval_start_instant: Instant,
    current_measurement_interval_total_time: Duration,
    current_measurement_interval_total_frames: u32,
    reported_frame_time_capped: Option<Duration>,
    reported_frame_time_uncapped: Option<Duration>//estimate of what the frame time would be without sleeping
}
impl MainLoopClock {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            current_frame_start_instant: now,
            current_measurement_interval_start_instant: now,
            current_measurement_interval_total_time: Duration::ZERO,
            current_measurement_interval_total_frames: 0,
            reported_frame_time_capped: None,
            reported_frame_time_uncapped: None
        }
    }

    /// returns true if end of measurement interval
    pub fn tick(&mut self, max_fps: f32) -> bool {
        let now = Instant::now();

        let frame_duration = now - self.current_frame_start_instant;
        self.current_measurement_interval_total_time += frame_duration;
        self.current_measurement_interval_total_frames += 1;

        let is_end_of_measurement_interval = now - self.current_measurement_interval_start_instant > MEASUREMENT_INTERVAL_DURATION;
        if is_end_of_measurement_interval {
            self.conclude_measurement_interval();
        }

        let minimum_frame_time = Duration::from_secs_f32(1.0 / max_fps);
        let earliest_allowed_frame_end_instant = self.current_frame_start_instant + minimum_frame_time;
        spin_sleep::sleep(earliest_allowed_frame_end_instant - now);

        self.current_frame_start_instant = Instant::now();

        is_end_of_measurement_interval
    }

    // pub fn start_frame(&mut self) {
    //     self.current_frame_start_instant = Instant::now();

    //     // assert!(!self.waiting_for_frame_end);
    //     // self.waiting_for_frame_end = true;
    // }

    // pub fn end_frame(&mut self) {
    //     let now = Instant::now();
    //     let frame_duration = now - self.current_frame_start_instant;
    //     self.current_measurement_interval_total_time += frame_duration;
    //     self.current_measurement_interval_total_frames += 1;

    //     // assert!(self.waiting_for_frame_end);
    //     // self.waiting_for_frame_end = false;

    //     if now - self.current_measurement_interval_start_instant > MEASUREMENT_INTERVAL_DURATION {
    //         self.conclude_measurement_interval();
    //     }
    // }

    // pub fn average_fps_capped(&self) -> f32 {
    //     match self.reported_frame_time_capped {
    //         Some(time) => 1.0 / time.as_secs_f32(),
    //         None => f32::NAN
    //     }
    // }

    // pub fn average_fps_uncapped(&self) -> f32 {
    //     match self.reported_frame_time_uncapped {
    //         Some(time) => 1.0 / time.as_secs_f32(),
    //         None => f32::NAN
    //     }
    // }

    // pub fn average_milli_seconds_per_frame_capped(&self) -> f32 {
    //     match self.reported_frame_time_capped {
    //         Some(time) => time.as_secs_f32() * 1000.0,
    //         None => f32::NAN
    //     }
    // }

    // pub fn average_milli_seconds_per_frame_uncapped(&self) -> f32 {
    //     match self.reported_frame_time_uncapped {
    //         Some(time) => time.as_secs_f32() * 1000.0,
    //         None => f32::NAN
    //     }
    // }

    pub fn average_frame_timgings(&self) -> AverageFrameTimings {
        AverageFrameTimings {
            capped_fps: self.reported_frame_time_capped.map_or(f32::NAN, |t| 1.0 / t.as_secs_f32()),
            uncapped_fps: self.reported_frame_time_uncapped.map_or(f32::NAN, |t| 1.0 / t.as_secs_f32()),
            capped_milliseconds_per_frame: self.reported_frame_time_capped.map_or(f32::NAN, |t| t.as_secs_f32() * 1000.0),
            uncapped_milliseconds_per_frame: self.reported_frame_time_uncapped.map_or(f32::NAN, |t| t.as_secs_f32() * 1000.0)
        }
    }

    fn conclude_measurement_interval(&mut self) {
        let now = Instant::now();

        self.reported_frame_time_capped = Some((now - self.current_measurement_interval_start_instant) / self.current_measurement_interval_total_frames);
        self.reported_frame_time_uncapped = Some(self.current_measurement_interval_total_time / self.current_measurement_interval_total_frames);

        //reset
        self.current_measurement_interval_start_instant = now;
        self.current_measurement_interval_total_time = Duration::ZERO;
        self.current_measurement_interval_total_frames = 0;
    }
}

pub struct AverageFrameTimings {
    pub capped_fps: f32,
    pub uncapped_fps: f32,
    pub capped_milliseconds_per_frame: f32,
    pub uncapped_milliseconds_per_frame: f32
}
impl AverageFrameTimings {
    pub fn new_nan() -> Self {
        Self {
            capped_fps: f32::NAN,
            uncapped_fps: f32::NAN,
            capped_milliseconds_per_frame: f32::NAN,
            uncapped_milliseconds_per_frame: f32::NAN
        }
    }
}