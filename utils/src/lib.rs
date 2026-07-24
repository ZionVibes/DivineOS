//! DivineOS Utility Crates
//! 
//! This crate provides shared utility components used across the DivineOS
//! system. These utilities include common data structures, error handling,
//! logging, and other foundational elements that support the core system
//! functionality.

// Re-export common utilities
pub use crate::error::DivineError;
pub use crate::logging::setup_logging;
pub use crate::config::Config;
pub use crate::timer::SystemTimer;
pub use crate::sync::MutexGuard;

// Utility modules
pub mod error;
pub mod logging;
pub mod config;
pub mod timer;
pub mod sync;
pub mod crypto;
pub mod serialization;

// Common data structures and types
pub mod types {
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    
    /// System time type
    pub type SystemTime = u64;
    
    /// Result type for DivineOS operations
    pub type Result<T> = std::result::Result<T, DivineError>;
    
    /// System status enumeration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SystemStatus {
        /// System is initializing
        Initializing,
        /// System is running normally
        Running,
        /// System is in maintenance mode
        Maintenance,
        /// System is shutting down
        ShuttingDown,
        /// System is in error state
        Error,
    }
    
    /// Resource usage statistics
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceStats {
        /// CPU usage percentage
        pub cpu_percent: f64,
        /// Memory usage in MB
        pub memory_mb: u64,
        /// Disk usage in MB
        pub disk_mb: u64,
        /// Network usage in bytes
        pub network_bytes: u64,
        /// Timestamp of measurement
        pub timestamp: SystemTime,
    }
    
    /// System health indicator
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SystemHealth {
        /// Overall system health score (0.0 to 1.0)
        pub health_score: f64,
        /// Critical components status
        pub critical_components: Vec<ComponentStatus>,
        /// Resource utilization
        pub resource_usage: ResourceStats,
        /// System uptime
        pub uptime: Duration,
        /// Timestamp of health check
        pub timestamp: SystemTime,
    }
    
    /// Component status
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ComponentStatus {
        /// Component name
        pub name: String,
        /// Component status
        pub status: ComponentStatusEnum,
        /// Last updated timestamp
        pub last_updated: SystemTime,
        /// Additional details
        pub details: String,
    }
    
    /// Component status enumeration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ComponentStatusEnum {
        /// Component is healthy
        Healthy,
        /// Component is degraded
        Degraded,
        /// Component is unhealthy
        Unhealthy,
        /// Component is offline
        Offline,
    }
}

// Utility configuration
pub mod config {
    use serde::{Deserialize, Serialize};
    
    /// Base configuration structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        /// Application name
        pub app_name: String,
        /// Application version
        pub version: String,
        /// Logging level
        pub log_level: LogLevel,
        /// Debug mode
        pub debug: bool,
        /// Configuration file path
        pub config_file: Option<String>,
    }
    
    /// Logging level enumeration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum LogLevel {
        /// Trace level (most verbose)
        Trace,
        /// Debug level
        Debug,
        /// Info level (default)
        Info,
        /// Warning level
        Warn,
        /// Error level (least verbose)
        Error,
    }
    
    impl Default for Config {
        fn default() -> Self {
            Self {
                app_name: "DivineOS".to_string(),
                version: "0.1.0".to_string(),
                log_level: LogLevel::Info,
                debug: false,
                config_file: None,
            }
        }
    }
}

// Error handling utilities
pub mod error {
    use serde::{Deserialize, Serialize};
    
    /// Core DivineOS error type
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DivineError {
        /// Configuration error
        ConfigError(String),
        /// System error
        SystemError(String),
        /// IO error
        IoError(String),
        /// Serialization error
        SerializationError(String),
        /// Timeout error
        TimeoutError(String),
        /// Permission error
        PermissionError(String),
        /// Resource error
        ResourceError(String),
        /// Internal error
        InternalError(String),
    }
    
    impl std::fmt::Display for DivineError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DivineError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
                DivineError::SystemError(msg) => write!(f, "System error: {}", msg),
                DivineError::IoError(msg) => write!(f, "IO error: {}", msg),
                DivineError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
                DivineError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
                DivineError::PermissionError(msg) => write!(f, "Permission error: {}", msg),
                DivineError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
                DivineError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for DivineError {}
    
    impl From<std::io::Error> for DivineError {
        fn from(error: std::io::Error) -> Self {
            DivineError::IoError(error.to_string())
        }
    }
    
    impl From<serde_json::Error> for DivineError {
        fn from(error: serde_json::Error) -> Self {
            DivineError::SerializationError(error.to_string())
        }
    }
}

// Logging utilities
pub mod logging {
    use log::{LevelFilter, set_logger, set_max_level};
    use std::sync::Once;
    
    /// Initialize logging system
    pub fn setup_logging(log_level: log::LevelFilter) {
        static INIT: Once = Once::new();
        
        INIT.call_once(|| {
            env_logger::Builder::from_default_env()
                .filter_level(log_level)
                .init();
        });
    }
    
    /// Get current log level
    pub fn get_log_level() -> LevelFilter {
        env_logger::from_env(env_logger::Env::default()).filter_level()
    }
}

// Timer utilities
pub mod timer {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    /// System timer for measuring time intervals
    #[derive(Debug, Clone)]
    pub struct SystemTimer {
        start_time: std::time::Instant,
    }
    
    impl SystemTimer {
        /// Create a new timer
        pub fn new() -> Self {
            Self {
                start_time: std::time::Instant::now(),
            }
        }
        
        /// Get elapsed time since timer creation
        pub fn elapsed(&self) -> std::time::Duration {
            self.start_time.elapsed()
        }
        
        /// Reset the timer
        pub fn reset(&mut self) {
            self.start_time = std::time::Instant::now();
        }
    }
    
    /// Get current system time in milliseconds
    pub fn current_time_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
    
    /// Get current system time in seconds
    pub fn current_time_s() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

// Synchronization utilities
pub mod sync {
    use std::sync::Mutex;
    
    /// Wrapper for mutex guard with additional functionality
    pub struct MutexGuard<T> {
        guard: std::sync::MutexGuard<'static, T>,
    }
    
    impl<T> MutexGuard<T> {
        /// Create a new mutex guard
        pub fn new(mutex: &Mutex<T>) -> Self {
            // This is a simplified implementation - in practice, you'd need
            // proper lifetime management for the guard
            todo!("This is a placeholder - proper implementation would require more complex synchronization")
        }
    }
    
    /// Simple lock-free counter for atomic operations
    #[derive(Debug)]
    pub struct AtomicCounter {
        value: std::sync::atomic::AtomicU64,
    }
    
    impl AtomicCounter {
        /// Create a new atomic counter
        pub fn new(initial_value: u64) -> Self {
            Self {
                value: std::sync::atomic::AtomicU64::new(initial_value),
            }
        }
        
        /// Increment the counter
        pub fn increment(&self) -> u64 {
            self.value.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        }
        
        /// Get current value
        pub fn get(&self) -> u64 {
            self.value.load(std::sync::atomic::Ordering::Relaxed)
        }
    }
}

// Cryptographic utilities
pub mod crypto {
    use sha2::{Sha256, Digest};
    
    /// Hash a string using SHA-256
    pub fn hash_string(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    
    /// Generate a random string
    pub fn random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
            .collect()
    }
}

// Serialization utilities
pub mod serialization {
    use serde::{Serialize, Deserialize};
    
    /// Serialize data to JSON
    pub fn to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string(data)
    }
    
    /// Deserialize data from JSON
    pub fn from_json<T: DeserializeOwned>(json: &str) -> Result<T, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// Serialize data to binary format
    pub fn to_binary<T: Serialize>(data: &T) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(data)
    }
    
    /// Deserialize data from binary format
    pub fn from_binary<T: DeserializeOwned>(data: &[u8]) -> Result<T, bincode::Error> {
        bincode::deserialize(data)
    }
}