use crate::metric_name::{BuildPerformanceMetricName, BuildTimeMetricName};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileStatisticsData<'r> {
    pub version: i64,
    pub project_name: &'r str,
    pub task_name: &'r str,
    pub task_result: &'r str,
    pub duration_ms: i64,
    pub tags: Vec<&'r str>,
    pub changes: Vec<&'r str>,
    pub build_uuid: &'r str,
    pub kotlin_version: &'r str,
    pub host_name: &'r str,
    pub finish_time: i64,
    pub timestamp: &'r str,
    pub compiler_arguments: Vec<String>,
    pub non_incremental_attributes: Vec<&'r str>,
    pub build_times_metrics: HashMap<BuildTimeMetricName, i64>,
    pub performance_metrics: HashMap<BuildPerformanceMetricName, i64>,
}
