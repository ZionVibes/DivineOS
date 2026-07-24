//! DivineOS IPC Library - Inter-process communication framework
//! 
//! This crate provides the communication infrastructure between the
//! deterministic kernel and AI runtime components. It implements both
//! synchronous and asynchronous communication patterns with appropriate
//! security and performance characteristics.
//! 
//! The IPC system supports:
//! 
//! 1. Synchronous RPC calls for critical deterministic operations
//! 2. Asynchronous message queues for non-critical AI operations
//! 3. Shared memory regions for high-frequency data exchange
//! 4. Event bus for system-wide notifications
//! 
//! All communication is designed to maintain the deterministic guarantees
//! required by the kernel while enabling efficient AI-driven optimization.

// Re-export core IPC components
pub use crate::rpc::RpcClient;
pub use crate::rpc::RpcServer;
pub use crate::message_queue::MessageQueue;
pub use crate::event_bus::EventBus;
pub use crate::shared_memory::SharedMemoryManager;

// IPC modules
pub mod rpc;
pub mod message_queue;
pub mod event_bus;
pub mod shared_memory;
pub mod types;

// Core IPC types and error definitions
pub mod error {
    use serde::{Deserialize, Serialize};
    
    /// IPC error types
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum IpcError {
        /// Connection error
        ConnectionError(String),
        
        /// Serialization error
        SerializationError(String),
        
        /// Timeout error
        TimeoutError(String),
        
        /// Permission denied
        PermissionDenied(String),
        
        /// Resource exhausted
        ResourceExhausted(String),
        
        /// Internal error
        InternalError(String),
    }
    
    impl std::fmt::Display for IpcError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                IpcError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
                IpcError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
                IpcError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
                IpcError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
                IpcError::ResourceExhausted(msg) => write!(f, "Resource exhausted: {}", msg),
                IpcError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for IpcError {}
}

// IPC configuration
pub mod config {
    /// IPC configuration parameters
    #[derive(Debug, Clone)]
    pub struct IpcConfig {
        /// Maximum message size
        pub max_message_size: usize,
        
        /// RPC timeout duration
        pub rpc_timeout: std::time::Duration,
        
        /// Message queue capacity
        pub queue_capacity: usize,
        
        /// Shared memory size
        pub shared_memory_size: usize,
        
        /// Security level
        pub security_level: SecurityLevel,
    }
    
    /// Security level for IPC
    #[derive(Debug, Clone)]
    pub enum SecurityLevel {
        /// Minimal security (development/testing)
        Minimal,
        /// Standard security (production)
        Standard,
        /// High security (sensitive environments)
        High,
    }
    
    impl Default for IpcConfig {
        fn default() -> Self {
            Self {
                max_message_size: 1024 * 1024, // 1MB
                rpc_timeout: std::time::Duration::from_millis(500),
                queue_capacity: 1000,
                shared_memory_size: 1024 * 1024 * 100, // 100MB
                security_level: SecurityLevel::Standard,
            }
        }
    }
}

// IPC initialization and startup
pub mod startup {
    use super::error::IpcError;
    
    /// Initialize the IPC system
    pub fn initialize_ipc() -> Result<(), IpcError> {
        // TODO: Implement IPC initialization sequence
        // This would include:
        // 1. Setting up RPC communication channels
        // 2. Initializing message queues
        // 3. Setting up shared memory regions
        // 4. Starting event bus
        // 5. Configuring security policies
        
        Ok(())
    }
    
    /// Start the IPC services
    pub fn start_ipc() -> Result<(), IpcError> {
        // TODO: Implement IPC startup sequence
        // This would include:
        // 1. Starting RPC servers
        // 2. Starting message queue processors
        // 3. Starting event bus subscribers
        // 4. Initializing security components
        
        Ok(())
    }
}