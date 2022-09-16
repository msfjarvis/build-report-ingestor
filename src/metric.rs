/// The structs defined here match their `enum class` counterparts
/// in the reference Kotlin source code. Since Rust enums lack the
/// variant type system exposed by JVM enums, we're instead using
/// a struct and associated constants to provide the concrete
/// implementations.

pub struct BuildTime<'a> {
    pub parent: Option<&'a BuildTime<'a>>,
    pub description: &'a str,
}

impl BuildTime<'_> {
    pub const GRADLE_TASK: BuildTime<'static> = BuildTime {
        parent: None,
        description: "Total Gradle task time",
    };

    pub const GRADLE_TASK_ACTION: BuildTime<'static> = BuildTime {
        parent: None,
        description: "Task action",
    };

    pub const CLEAR_OUTPUT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::GRADLE_TASK_ACTION),
        description: "Clear output",
    };

    pub const BACKUP_OUTPUT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::GRADLE_TASK_ACTION),
        description: "Backup output",
    };

    pub const RESTORE_OUTPUT_FROM_BACKUP: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::GRADLE_TASK_ACTION),
        description: "Restore output",
    };

    pub const CONNECT_TO_DAEMON: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::GRADLE_TASK_ACTION),
        description: "Connect to Kotlin daemon",
    };

    pub const CLEAR_JAR_CACHE: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::GRADLE_TASK_ACTION),
        description: "Clear jar cache",
    };

    pub const CALCULATE_OUTPUT_SIZE: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::GRADLE_TASK_ACTION),
        description: "Calculate output size",
    };

    pub const RUN_COMPILATION: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::GRADLE_TASK_ACTION),
        description: "Run compilation",
    };

    pub const NON_INCREMENTAL_COMPILATION_IN_PROCESS: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::RUN_COMPILATION),
        description: "Non incremental inprocess compilation",
    };

    pub const NON_INCREMENTAL_COMPILATION_OUT_OF_PROCESS: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::RUN_COMPILATION),
        description: "Non incremental out of process compilation",
    };

    pub const NON_INCREMENTAL_COMPILATION_DAEMON: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::RUN_COMPILATION),
        description: "Non incremental compilation in daemon",
    };

    pub const INCREMENTAL_COMPILATION_DAEMON: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::RUN_COMPILATION),
        description: "Incremental compilation in daemon",
    };

    pub const STORE_BUILD_INFO: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
        description: "Store build info",
    };

    pub const JAR_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
        description: "ABI JAR Snapshot support",
    };

    pub const SET_UP_ABI_SNAPSHOTS: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::JAR_SNAPSHOT),
        description: "Set up ABI snapshot",
    };

    pub const IC_ANALYZE_JAR_FILES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::JAR_SNAPSHOT),
        description: "Analyze jar files",
    };

    pub const IC_CALCULATE_INITIAL_DIRTY_SET: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
        description: "Calculate initial dirty sources set",
    };

    pub const COMPUTE_CLASSPATH_CHANGES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::IC_CALCULATE_INITIAL_DIRTY_SET),
        description: "Compute classpath changes",
    };

    pub const LOAD_CURRENT_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CLASSPATH_CHANGES),
        description: "Load current classpath snapshot",
    };

    pub const REMOVE_DUPLICATE_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::LOAD_CURRENT_CLASSPATH_SNAPSHOT),
        description: "Remove duplicate classes",
    };

    pub const SHRINK_CURRENT_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CLASSPATH_CHANGES),
        description: "Shrink current classpath snapshot",
    };
    pub const GET_LOOKUP_SYMBOLS: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SHRINK_CURRENT_CLASSPATH_SNAPSHOT),
        description: "Get lookup symbols",
    };
    pub const FIND_REFERENCED_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SHRINK_CURRENT_CLASSPATH_SNAPSHOT),
        description: "Find referenced classes",
    };
    pub const FIND_TRANSITIVELY_REFERENCED_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SHRINK_CURRENT_CLASSPATH_SNAPSHOT),
        description: "Find transitively referenced classes",
    };
    pub const LOAD_SHRUNK_PREVIOUS_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CLASSPATH_CHANGES),
        description: "Load shrunk previous classpath snapshot",
    };
    pub const COMPUTE_CHANGED_AND_IMPACTED_SET: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CLASSPATH_CHANGES),
        description: "Compute changed and impacted set",
    };
    pub const COMPUTE_CLASS_CHANGES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CHANGED_AND_IMPACTED_SET),
        description: "Compute class changes",
    };
    pub const COMPUTE_KOTLIN_CLASS_CHANGES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CLASS_CHANGES),
        description: "Compute Kotlin class changes",
    };
    pub const COMPUTE_JAVA_CLASS_CHANGES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CLASS_CHANGES),
        description: "Compute Java class changes",
    };
    pub const COMPUTE_IMPACTED_SET: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPUTE_CHANGED_AND_IMPACTED_SET),
        description: "Compute impacted set",
    };
    pub const IC_ANALYZE_CHANGES_IN_DEPENDENCIES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::IC_CALCULATE_INITIAL_DIRTY_SET),
        description: "Analyze dependency changes",
    };
    pub const IC_FIND_HISTORY_FILES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::IC_ANALYZE_CHANGES_IN_DEPENDENCIES),
        description: "Find history files",
    };
    pub const IC_ANALYZE_HISTORY_FILES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::IC_ANALYZE_CHANGES_IN_DEPENDENCIES),
        description: "Analyze history files",
    };
    pub const IC_ANALYZE_CHANGES_IN_JAVA_SOURCES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::IC_CALCULATE_INITIAL_DIRTY_SET),
        description: "Analyze Java file changes",
    };
    pub const IC_ANALYZE_CHANGES_IN_ANDROID_LAYOUTS: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::IC_CALCULATE_INITIAL_DIRTY_SET),
        description: "Analyze Android layouts",
    };
    pub const IC_DETECT_REMOVED_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::IC_CALCULATE_INITIAL_DIRTY_SET),
        description: "Detect removed classes",
    };
    pub const CLEAR_OUTPUT_ON_REBUILD: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
        description: "Clear outputs on rebuild",
    };
    pub const IC_UPDATE_CACHES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
        description: "Update caches",
    };
    pub const COMPILATION_ROUND: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
        description: "Sources compilation round",
    };
    pub const IC_WRITE_HISTORY_FILE: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
        description: "Write history file",
    };
    pub const SHRINK_AND_SAVE_CURRENT_CLASSPATH_SNAPSHOT_AFTER_COMPILATION: BuildTime<'static> =
        BuildTime {
            parent: Some(&BuildTime::INCREMENTAL_COMPILATION_DAEMON),
            description: "Shrink and save current classpath snapshot after compilation",
        };
    pub const INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SHRINK_AND_SAVE_CURRENT_CLASSPATH_SNAPSHOT_AFTER_COMPILATION),
        description: "Shrink current classpath snapshot incrementally",
    };
    pub const INCREMENTAL_LOAD_CURRENT_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT),
        description: "Load current classpath snapshot",
    };
    pub const INCREMENTAL_LOAD_SHRUNK_CURRENT_CLASSPATH_SNAPSHOT_AGAINST_PREVIOUS_LOOKUPS:
        BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT),
        description: "Load shrunk current classpath snapshot against previous lookups",
    };
    pub const NON_INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SHRINK_AND_SAVE_CURRENT_CLASSPATH_SNAPSHOT_AFTER_COMPILATION),
        description: "Shrink current classpath snapshot non-incrementally",
    };
    pub const NON_INCREMENTAL_LOAD_CURRENT_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::NON_INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT),
        description: "Load current classpath snapshot",
    };
    pub const SAVE_SHRUNK_CURRENT_CLASSPATH_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SHRINK_AND_SAVE_CURRENT_CLASSPATH_SNAPSHOT_AFTER_COMPILATION),
        description: "Save shrunk current classpath snapshot",
    };
    pub const COMPILER_PERFORMANCE: BuildTime<'static> = BuildTime {
        parent: None,
        description: "Compiler performance",
    };
    pub const COMPILER_INITIALIZATION: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPILER_PERFORMANCE),
        description: "Compiler initialization time",
    };
    pub const CODE_ANALYSIS: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPILER_PERFORMANCE),
        description: "Compiler code analysis",
    };
    pub const CODE_GENERATION: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::COMPILER_PERFORMANCE),
        description: "Compiler code generation",
    };
    pub const CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM: BuildTime<'static> = BuildTime {
        parent: None,
        description: "Classpath entry snapshot transform",
    };
    pub const LOAD_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM),
        description: "Load classes",
    };
    pub const SNAPSHOT_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM),
        description: "Snapshot classes",
    };
    pub const READ_CLASSES_BASIC_INFO: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SNAPSHOT_CLASSES),
        description: "Read basic information about classes",
    };
    pub const FIND_INACCESSIBLE_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SNAPSHOT_CLASSES),
        description: "Find inaccessible classes",
    };
    pub const SNAPSHOT_KOTLIN_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SNAPSHOT_CLASSES),
        description: "Snapshot Kotlin classes",
    };
    pub const SNAPSHOT_JAVA_CLASSES: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::SNAPSHOT_CLASSES),
        description: "Snapshot Java classes",
    };
    pub const SAVE_CLASSPATH_ENTRY_SNAPSHOT: BuildTime<'static> = BuildTime {
        parent: Some(&BuildTime::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM),
        description: "Save classpath entry snapshot",
    };
}

pub enum SizeMetricType {
    BYTES,
    NUMBER,
}

pub struct BuildPerformanceMetric<'a> {
    pub parent: Option<&'a BuildPerformanceMetric<'a>>,
    pub description: &'a str,
    pub metric_type: SizeMetricType,
}

impl BuildPerformanceMetric<'_> {
    pub const CACHE_DIRECTORY_SIZE: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: None,
        description: "Total size of the cache directory",
        metric_type: SizeMetricType::BYTES,
    };

    pub const LOOKUP_SIZE: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: Some(&BuildPerformanceMetric::CACHE_DIRECTORY_SIZE),
        description: "Lookups size",
        metric_type: SizeMetricType::BYTES,
    };

    pub const SNAPSHOT_SIZE: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: Some(&BuildPerformanceMetric::CACHE_DIRECTORY_SIZE),
        description: "ABI snapshot size",
        metric_type: SizeMetricType::BYTES,
    };

    pub const BUNDLE_SIZE: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: None,
        description: "Total size of the final bundle",
        metric_type: SizeMetricType::BYTES,
    };

    pub const COMPILE_ITERATION: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: None,
        description: "Total compiler iteration",
        metric_type: SizeMetricType::NUMBER,
    };

    pub const CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM_EXECUTION_COUNT: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: None,
            description: "Number of times 'ClasspathEntrySnapshotTransform' ran",
            metric_type: SizeMetricType::NUMBER,
        };

    pub const JAR_CLASSPATH_ENTRY_SIZE: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: Some(&BuildPerformanceMetric::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM_EXECUTION_COUNT),
        description: "Size of jar classpath entry",
        metric_type: SizeMetricType::BYTES,
    };

    pub const JAR_CLASSPATH_ENTRY_SNAPSHOT_SIZE: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: Some(
                &BuildPerformanceMetric::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM_EXECUTION_COUNT,
            ),
            description: "Size of jar classpath entry's snapshot",
            metric_type: SizeMetricType::BYTES,
        };

    pub const DIRECTORY_CLASSPATH_ENTRY_SNAPSHOT_SIZE: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: Some(
                &BuildPerformanceMetric::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM_EXECUTION_COUNT,
            ),
            description: "Size of directory classpath entry's snapshot",
            metric_type: SizeMetricType::BYTES,
        };

    pub const COMPUTE_CLASSPATH_CHANGES_EXECUTION_COUNT: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: None,
            description: "Number of times classpath changes are computed",
            metric_type: SizeMetricType::NUMBER,
        };

    pub const SHRINK_AND_SAVE_CLASSPATH_SNAPSHOT_EXECUTION_COUNT: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: None,
            description: "Number of times classpath snapshot is shrunk and saved after compilation",
            metric_type: SizeMetricType::NUMBER,
        };

    pub const CLASSPATH_ENTRY_COUNT: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: Some(&BuildPerformanceMetric::SHRINK_AND_SAVE_CLASSPATH_SNAPSHOT_EXECUTION_COUNT),
        description: "Number of classpath entries",
        metric_type: SizeMetricType::NUMBER,
    };

    pub const CLASSPATH_SNAPSHOT_SIZE: BuildPerformanceMetric<'static> = BuildPerformanceMetric {
        parent: Some(&BuildPerformanceMetric::SHRINK_AND_SAVE_CLASSPATH_SNAPSHOT_EXECUTION_COUNT),
        description: "Size of classpath snapshot",
        metric_type: SizeMetricType::BYTES,
    };

    pub const SHRUNK_CLASSPATH_SNAPSHOT_SIZE: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: Some(
                &BuildPerformanceMetric::SHRINK_AND_SAVE_CLASSPATH_SNAPSHOT_EXECUTION_COUNT,
            ),
            description: "Size of shrunk classpath snapshot",
            metric_type: SizeMetricType::BYTES,
        };

    pub const LOAD_CLASSPATH_SNAPSHOT_EXECUTION_COUNT: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: None,
            description: "Number of times classpath snapshot is loaded",
            metric_type: SizeMetricType::NUMBER,
        };

    pub const LOAD_CLASSPATH_ENTRY_SNAPSHOT_CACHE_HITS: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: Some(&BuildPerformanceMetric::LOAD_CLASSPATH_SNAPSHOT_EXECUTION_COUNT),
            description: "Number of cache hits when loading classpath entry snapshots",
            metric_type: SizeMetricType::NUMBER,
        };

    pub const LOAD_CLASSPATH_ENTRY_SNAPSHOT_CACHE_MISSES: BuildPerformanceMetric<'static> =
        BuildPerformanceMetric {
            parent: Some(&BuildPerformanceMetric::LOAD_CLASSPATH_SNAPSHOT_EXECUTION_COUNT),
            description: "Number of cache misses when loading classpath entry snapshots",
            metric_type: SizeMetricType::NUMBER,
        };
}
