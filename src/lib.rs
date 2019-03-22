use std::time::{Duration, SystemTime};

pub struct Timer {
    started: Option<SystemTime>,
    laps: Vec<Duration>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            started: None,
            laps: vec![],
        }
    }

    /// # Examples:
    ///
    /// ```
    /// use yatl::Timer;
    ///
    /// let mut timer = Timer::new();
    /// assert_eq!(Ok(()), timer.start());
    /// assert_eq!(true, timer.start().is_err());
    /// ```
    pub fn start(&mut self) -> Result<(), &str> {
        match self.started {
            None => {
                self.started = Some(SystemTime::now());
                Ok(())
            }
            Some(_) => Err("Timer already started!")
        }
    }

    /// # Examples:
    ///
    /// ```
    /// use yatl::Timer;
    ///
    /// let mut timer = Timer::new();
    /// assert_eq!(true, timer.start_time().is_err(), "Can get start time before starting the timer!?");
    /// timer.start();
    /// assert_eq!(true, timer.start_time().is_ok());
    /// ```
    pub fn start_time(&self) -> Result<SystemTime, &str> {
        match self.started {
            Some(s) => Ok(s.clone()),
            None => Err("Timer not started!")
        }
    }

    /// # Examples:
    ///
    /// ```
    /// use yatl::Timer;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let mut timer = Timer::new();
    /// assert_eq!(true, timer.lap().is_err());
    /// timer.start();
    /// sleep(Duration::from_micros(10));
    /// let lap = timer.lap();
    /// assert_eq!(true, lap.is_ok());
    /// assert_eq!(true, lap.unwrap().as_nanos() > 0, "No time passed?!");
    /// ```
    pub fn lap(&mut self) -> Result<Duration, String> {
        match self.started {
            Some(s) => {
                match s.elapsed() {
                    Ok(e) => {
                        self.laps.push(e);
                        Ok(e.clone())
                    }
                    Err(e) =>
                    Err(format!("Internal Error: {:?}", e))
                }
            }
            None => Err("Timer not started!".to_string())
        }
    }

    /// # Examples:
    ///
    /// ```
    /// use yatl::Timer;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let mut timer = Timer::new();
    /// let mut laps: Vec<Duration> = vec![];
    ///
    /// timer.start();
    /// laps.push(timer.lap().unwrap());
    /// sleep(Duration::from_micros(10));
    /// laps.push(timer.lap().unwrap());
    /// assert_eq!(laps, timer.laps())
    /// ```
    pub fn laps(&self) -> Vec<Duration> {
        self.laps.clone()
    }

    pub fn laps_formatted(&self) -> Vec<String> {
        let formatted: Vec<String> = self.laps.iter().map(|d| duration_to_human_string(d)).collect();
        formatted
    }
}

/// # Examples:
///
/// ## Formatted as nano seconds:
/// ```
/// use yatl::duration_to_human_string;
/// use std::time::Duration;
///
/// assert_eq!("12ns", duration_to_human_string(&Duration::from_nanos(12)));
/// ```
///
/// ## Formatted as micro seconds:
/// ```
/// use yatl::duration_to_human_string;
/// use std::time::Duration;
///
/// assert_eq!("13us", duration_to_human_string(&Duration::from_nanos(13674)));
/// ```
///
/// ## Formatted as milli seconds:
/// ```
/// use yatl::duration_to_human_string;
/// use std::time::Duration;
///
/// assert_eq!("45ms", duration_to_human_string(&Duration::from_nanos(45674432)));
/// ```
///
/// ## Formatted as seconds:
/// ```
/// use yatl::duration_to_human_string;
/// use std::time::Duration;
///
/// assert_eq!("2s", duration_to_human_string(&Duration::from_nanos(2746859738)));
/// ```
///
/// ## Formatted as minutes:
/// ```
/// use yatl::duration_to_human_string;
/// use std::time::Duration;
///
/// assert_eq!("13m", duration_to_human_string(&Duration::from_nanos(780897563728)));
/// ```
pub fn duration_to_human_string(duration: &Duration) -> String {
    return if duration.as_nanos() < 1000 {
        format!("{}ns", duration.as_nanos())
    } else if duration.as_micros() < 1000 {
        format!("{}us", duration.as_micros())
    } else if duration.as_millis() < 1000 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_secs() < 60 {
        format!("{}s", duration.as_secs())
    }else {
        format!("{}m", duration.as_secs() / 60)
    }
}