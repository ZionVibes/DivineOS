//! Interface contract between deterministic kernel and AI runtime
//! This defines the safe communication boundaries between the two components

use std::collections::HashMap;
use std::time::Duration;

// Core trait for kernel APIs exposed to AI
pub trait KernelInterface {
    /// Get system resource usage statistics
    fn get_resource_stats(&self) -> ResourceStats;
    
    /// Get process information
    fn get_process_info(&self, pid: ProcessId) -> Result<ProcessInfo, KernelError>;
    
    /// Get system health metrics
    fn get_system_health(&self) -> SystemHealth;
    
    /// Request process scheduling decision
    fn request_scheduling_decision(&self, context: SchedulingContext) -> Result<SchedulingDecision, KernelError>;
    
    /// Request memory allocation
    fn request_memory_allocation(&self, request: MemoryRequest) -> Result<MemoryAllocation, KernelError>;
    
    /// Request security policy evaluation
    fn evaluate_security_policy(&self, policy: SecurityPolicy) -> Result<SecurityDecision, KernelError>;
    
    /// Notify kernel of AI decision
    fn notify_ai_decision(&self, decision: AIDecision) -> Result<(), KernelError>;
}

// Core trait for AI APIs exposed to kernel services
pub trait AIInterface {
    /// Get AI model performance metrics
    fn get_ai_performance(&self) -> AIPerformance;
    
    /// Request AI optimization recommendation
    fn request_optimization(&self, context: OptimizationContext) -> Result<OptimizationRecommendation, AIError>;
    
    /// Get AI model status
    fn get_model_status(&self) -> ModelStatus;
    
    /// Update AI model with new data
    fn update_model(&self, data: ModelUpdate) -> Result<(), AIError>;
    
    /// Request AI decision for security
    fn request_security_decision(&self, context: SecurityContext) -> Result<SecurityDecision, AIError>;
}

// Resource statistics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStats {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub network_io: u64,
    pub timestamp: u64,
}

// Process information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: ProcessId,
    pub name: String,
    pub priority: Priority,
    pub memory_usage: u64,
    pub cpu_time: Duration,
    pub status: ProcessStatus,
    pub parent_pid: Option<ProcessId>,
}

// System health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub cpu_temperature: f32,
    pub memory_pressure: f64,
    pub disk_health: DiskHealth,
    pub network_status: NetworkStatus,
    pub timestamp: u64,
}

// Scheduling context for AI decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingContext {
    pub process_id: ProcessId,
    pub priority: Priority,
    pub resource_requirements: ResourceRequirements,
    pub historical_performance: Vec<ProcessPerformance>,
    pub system_load: SystemLoad,
    pub timestamp: u64,
}

// Scheduling decision from AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingDecision {
    pub process_id: ProcessId,
    pub recommended_priority: Priority,
    pub execution_time: Duration,
    pub resource_allocation: ResourceAllocation,
    pub confidence: f64,
    pub timestamp: u64,
}

// Memory request from AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRequest {
    pub process_id: ProcessId,
    pub size: u64,
    pub allocation_type: MemoryType,
    pub priority: Priority,
    pub timeout: Duration,
}

// Memory allocation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAllocation {
    pub process_id: ProcessId,
    pub address: u64,
    pub size: u64,
    pub allocation_type: MemoryType,
    pub timestamp: u64,
}

// Security policy evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub process_id: ProcessId,
    pub operation: SecurityOperation,
    pub resource_access: ResourceAccess,
    pub timestamp: u64,
}

// Security decision from kernel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDecision {
    pub process_id: ProcessId,
    pub allowed: bool,
    pub reason: String,
    pub timestamp: u64,
}

// AI decision notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecision {
    pub decision_type: AIDecisionType,
    pub context: AIDecisionContext,
    pub timestamp: u64,
}

// AI performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPerformance {
    pub model_accuracy: f64,
    pub inference_time: Duration,
    pub resource_usage: ResourceUsage,
    pub confidence_score: f64,
    pub timestamp: u64,
}

// Optimization context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationContext {
    pub resource_type: ResourceType,
    pub current_usage: f64,
    pub historical_data: Vec<HistoricalDataPoint>,
    pub target_performance: PerformanceTarget,
    pub timestamp: u64,
}

// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub resource_type: ResourceType,
    pub recommended_action: OptimizationAction,
    pub confidence: f64,
    pub expected_improvement: f64,
    pub timestamp: u64,
}

// Model status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatus {
    pub model_version: String,
    pub last_updated: u64,
    pub training_status: TrainingStatus,
    pub accuracy: f64,
    pub performance: PerformanceMetrics,
}

// Model update data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUpdate {
    pub model_version: String,
    pub training_data: Vec<TrainingSample>,
    pub update_type: UpdateType,
    pub timestamp: u64,
}

// Security context for AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub process_id: ProcessId,
    pub threat_level: ThreatLevel,
    pub behavior_pattern: BehaviorPattern,
    pub timestamp: u64,
}

// Process lifecycle events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessLifecycleEvent {
    ProcessCreated(ProcessInfo),
    ProcessStarted(ProcessId),
    ProcessTerminated(ProcessId),
    ProcessSuspended(ProcessId),
    ProcessResumed(ProcessId),
}

// System events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    ProcessLifecycle(ProcessLifecycleEvent),
    ResourceUsage(ResourceStats),
    SystemHealth(SystemHealth),
    SecurityAlert(SecurityAlert),
    AIDecision(AIDecision),
    OptimizationResult(OptimizationRecommendation),
}

// Security alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub process_id: ProcessId,
    pub threat_level: ThreatLevel,
    pub alert_type: AlertType,
    pub description: String,
    pub timestamp: u64,
}

// Telemetry data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub event_type: TelemetryEventType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub source: String,
}

// Error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelError {
    InvalidProcessId(ProcessId),
    ResourceExhausted(String),
    SecurityViolation(String),
    Timeout(String),
    InternalError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIError {
    ModelNotReady(String),
    InferenceError(String),
    InvalidInput(String),
    Timeout(String),
    InternalError(String),
}

// Type definitions
pub type ProcessId = u32;
pub type Priority = u8;
pub type ThreatLevel = u8;
pub type ResourceId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessStatus {
    Running,
    Waiting,
    Blocked,
    Terminated,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Heap,
    Stack,
    Mmap,
    Shared,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityOperation {
    Read,
    Write,
    Execute,
    Network,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceAccess {
    File(String),
    Network(String),
    Memory(ResourceId),
    Device(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIDecisionType {
    Scheduling,
    Memory,
    Security,
    Optimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIDecisionContext {
    SchedulingDecision(SchedulingDecision),
    MemoryAllocation(MemoryAllocation),
    SecurityDecision(SecurityDecision),
    Optimization(OptimizationRecommendation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    CPU,
    Memory,
    Disk,
    Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationAction {
    Increase,
    Decrease,
    Maintain,
    Reallocate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingStatus {
    Training,
    Ready,
    Failed,
    Updating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateType {
    Full,
    Incremental,
    Validation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    Security,
    Performance,
    Resource,
    Health,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEventType {
    Process,
    Resource,
    Security,
    AI,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub disk_mb: u64,
    pub network_bandwidth: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPerformance {
    pub process_id: ProcessId,
    pub cpu_time: Duration,
    pub memory_usage: u64,
    pub execution_time: Duration,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLoad {
    pub cpu_load: f64,
    pub memory_load: f64,
    pub disk_load: f64,
    pub network_load: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub disk_mb: u64,
    pub network_bandwidth: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskHealth {
    pub status: DiskStatus,
    pub temperature: f32,
    pub error_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiskStatus {
    Healthy,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub status: NetworkStatusEnum,
    pub bandwidth_used: u64,
    pub latency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkStatusEnum {
    Online,
    Offline,
    Limited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_mb: u64,
    pub disk_mb: u64,
    pub network_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTarget {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub response_time: Duration,
    pub throughput: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDataPoint {
    pub timestamp: u64,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    pub features: Vec<f64>,
    pub label: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub process_id: ProcessId,
    pub pattern_type: PatternType,
    pub confidence: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Normal,
    Anomalous,
    Suspicious,
    Malicious,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub accuracy: f64,
    pub latency: Duration,
    pub throughput: u64,
}