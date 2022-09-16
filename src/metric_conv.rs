use crate::metric::{BuildPerformanceMetric, BuildTime};
use crate::metric_name::{BuildPerformanceMetricName, BuildTimeMetricName};
use std::convert::From;

impl From<BuildPerformanceMetricName> for BuildPerformanceMetric<'static> {
    fn from(name: BuildPerformanceMetricName) -> Self {
        match name {
            BuildPerformanceMetricName::CacheDirectorySize => {
                BuildPerformanceMetric::CACHE_DIRECTORY_SIZE
            }
            BuildPerformanceMetricName::LookupSize => BuildPerformanceMetric::LOOKUP_SIZE,
            BuildPerformanceMetricName::SnapshotSize => BuildPerformanceMetric::SNAPSHOT_SIZE,
            BuildPerformanceMetricName::BundleSize => BuildPerformanceMetric::BUNDLE_SIZE,
            BuildPerformanceMetricName::CompileIteration => {
                BuildPerformanceMetric::COMPILE_ITERATION
            }
            BuildPerformanceMetricName::ClasspathEntrySnapshotTransformExecutionCount => {
                BuildPerformanceMetric::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM_EXECUTION_COUNT
            }
            BuildPerformanceMetricName::JarClasspathEntrySize => {
                BuildPerformanceMetric::JAR_CLASSPATH_ENTRY_SIZE
            }
            BuildPerformanceMetricName::JarClasspathEntrySnapshotSize => {
                BuildPerformanceMetric::JAR_CLASSPATH_ENTRY_SNAPSHOT_SIZE
            }
            BuildPerformanceMetricName::DirectoryClasspathEntrySnapshotSize => {
                BuildPerformanceMetric::DIRECTORY_CLASSPATH_ENTRY_SNAPSHOT_SIZE
            }
            BuildPerformanceMetricName::ComputeClasspathChangesExecutionCount => {
                BuildPerformanceMetric::COMPUTE_CLASSPATH_CHANGES_EXECUTION_COUNT
            }
            BuildPerformanceMetricName::ShrinkAndSaveClasspathSnapshotExecutionCount => {
                BuildPerformanceMetric::SHRINK_AND_SAVE_CLASSPATH_SNAPSHOT_EXECUTION_COUNT
            }
            BuildPerformanceMetricName::ClasspathEntryCount => {
                BuildPerformanceMetric::CLASSPATH_ENTRY_COUNT
            }
            BuildPerformanceMetricName::ClasspathSnapshotSize => {
                BuildPerformanceMetric::CLASSPATH_SNAPSHOT_SIZE
            }
            BuildPerformanceMetricName::ShrunkClasspathSnapshotSize => {
                BuildPerformanceMetric::SHRUNK_CLASSPATH_SNAPSHOT_SIZE
            }
            BuildPerformanceMetricName::LoadClasspathSnapshotExecutionCount => {
                BuildPerformanceMetric::LOAD_CLASSPATH_SNAPSHOT_EXECUTION_COUNT
            }
            BuildPerformanceMetricName::LoadClasspathEntrySnapshotCacheHits => {
                BuildPerformanceMetric::LOAD_CLASSPATH_ENTRY_SNAPSHOT_CACHE_HITS
            }
            BuildPerformanceMetricName::LoadClasspathEntrySnapshotCacheMisses => {
                BuildPerformanceMetric::LOAD_CLASSPATH_ENTRY_SNAPSHOT_CACHE_MISSES
            }
        }
    }
}

impl From<BuildTimeMetricName> for BuildTime<'static> {
    fn from(name: BuildTimeMetricName) -> Self {
        match name {
            BuildTimeMetricName::GradleTask => BuildTime::GRADLE_TASK,
            BuildTimeMetricName::GradleTaskAction => BuildTime::GRADLE_TASK_ACTION,
            BuildTimeMetricName::ClearOutput => BuildTime::CLEAR_OUTPUT,
            BuildTimeMetricName::BackupOutput => BuildTime::BACKUP_OUTPUT,
            BuildTimeMetricName::RestoreOutputFromBackup => BuildTime::RESTORE_OUTPUT_FROM_BACKUP,
            BuildTimeMetricName::ConnectToDaemon => BuildTime::CONNECT_TO_DAEMON,
            BuildTimeMetricName::ClearJarCache => BuildTime::CLEAR_JAR_CACHE,
            BuildTimeMetricName::CalculateOutputSize => BuildTime::CALCULATE_OUTPUT_SIZE,
            BuildTimeMetricName::RunCompilation => BuildTime::RUN_COMPILATION,
            BuildTimeMetricName::NonIncrementalCompilationInProcess => BuildTime::NON_INCREMENTAL_COMPILATION_IN_PROCESS,
            BuildTimeMetricName::NonIncrementalCompilationOutOfProcess => BuildTime::NON_INCREMENTAL_COMPILATION_OUT_OF_PROCESS,
            BuildTimeMetricName::NonIncrementalCompilationDaemon => BuildTime::NON_INCREMENTAL_COMPILATION_DAEMON,
            BuildTimeMetricName::IncrementalCompilationDaemon => BuildTime::INCREMENTAL_COMPILATION_DAEMON,
            BuildTimeMetricName::StoreBuildInfo => BuildTime::STORE_BUILD_INFO,
            BuildTimeMetricName::JarSnapshot => BuildTime::JAR_SNAPSHOT,
            BuildTimeMetricName::SetUpAbiSnapshots => BuildTime::SET_UP_ABI_SNAPSHOTS,
            BuildTimeMetricName::IcAnalyzeJarFiles => BuildTime::IC_ANALYZE_JAR_FILES,
            BuildTimeMetricName::IcCalculateInitialDirtySet => BuildTime::IC_CALCULATE_INITIAL_DIRTY_SET,
            BuildTimeMetricName::ComputeClasspathChanges => BuildTime::COMPUTE_CLASSPATH_CHANGES,
            BuildTimeMetricName::LoadCurrentClasspathSnapshot => BuildTime::LOAD_CURRENT_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::RemoveDuplicateClasses => BuildTime::REMOVE_DUPLICATE_CLASSES,
            BuildTimeMetricName::ShrinkCurrentClasspathSnapshot => BuildTime::SHRINK_CURRENT_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::GetLookupSymbols => BuildTime::GET_LOOKUP_SYMBOLS,
            BuildTimeMetricName::FindReferencedClasses => BuildTime::FIND_REFERENCED_CLASSES,
            BuildTimeMetricName::FindTransitivelyReferencedClasses => BuildTime::FIND_TRANSITIVELY_REFERENCED_CLASSES,
            BuildTimeMetricName::LoadShrunkPreviousClasspathSnapshot => BuildTime::LOAD_SHRUNK_PREVIOUS_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::ComputeChangedAndImpactedSet => BuildTime::COMPUTE_CHANGED_AND_IMPACTED_SET,
            BuildTimeMetricName::ComputeClassChanges => BuildTime::COMPUTE_CLASS_CHANGES,
            BuildTimeMetricName::ComputeKotlinClassChanges => BuildTime::COMPUTE_KOTLIN_CLASS_CHANGES,
            BuildTimeMetricName::ComputeJavaClassChanges => BuildTime::COMPUTE_JAVA_CLASS_CHANGES,
            BuildTimeMetricName::ComputeImpactedSet => BuildTime::COMPUTE_IMPACTED_SET,
            BuildTimeMetricName::IcAnalyzeChangesInDependencies => BuildTime::IC_ANALYZE_CHANGES_IN_DEPENDENCIES,
            BuildTimeMetricName::IcFindHistoryFiles => BuildTime::IC_FIND_HISTORY_FILES,
            BuildTimeMetricName::IcAnalyzeHistoryFiles => BuildTime::IC_ANALYZE_HISTORY_FILES,
            BuildTimeMetricName::IcAnalyzeChangesInJavaSources => BuildTime::IC_ANALYZE_CHANGES_IN_JAVA_SOURCES,
            BuildTimeMetricName::IcAnalyzeChangesInAndroidLayouts => BuildTime::IC_ANALYZE_CHANGES_IN_ANDROID_LAYOUTS,
            BuildTimeMetricName::IcDetectRemovedClasses => BuildTime::IC_DETECT_REMOVED_CLASSES,
            BuildTimeMetricName::ClearOutputOnRebuild => BuildTime::CLEAR_OUTPUT_ON_REBUILD,
            BuildTimeMetricName::IcUpdateCaches => BuildTime::IC_UPDATE_CACHES,
            BuildTimeMetricName::CompilationRound => BuildTime::COMPILATION_ROUND,
            BuildTimeMetricName::IcWriteHistoryFile => BuildTime::IC_WRITE_HISTORY_FILE,
            BuildTimeMetricName::ShrinkAndSaveCurrentClasspathSnapshotAfterCompilation => BuildTime::SHRINK_AND_SAVE_CURRENT_CLASSPATH_SNAPSHOT_AFTER_COMPILATION,
            BuildTimeMetricName::IncrementalShrinkCurrentClasspathSnapshot => BuildTime::INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::IncrementalLoadCurrentClasspathSnapshot => BuildTime::INCREMENTAL_LOAD_CURRENT_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::IncrementalLoadShrunkCurrentClasspathSnapshotAgainstPreviousLookups => BuildTime::INCREMENTAL_LOAD_SHRUNK_CURRENT_CLASSPATH_SNAPSHOT_AGAINST_PREVIOUS_LOOKUPS,
            BuildTimeMetricName::NonIncrementalShrinkCurrentClasspathSnapshot => BuildTime::NON_INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::NonIncrementalLoadCurrentClasspathSnapshot => BuildTime::NON_INCREMENTAL_LOAD_CURRENT_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::SaveShrunkCurrentClasspathSnapshot => BuildTime::SAVE_SHRUNK_CURRENT_CLASSPATH_SNAPSHOT,
            BuildTimeMetricName::CompilerPerformance => BuildTime::COMPILER_PERFORMANCE,
            BuildTimeMetricName::CompilerInitialization => BuildTime::COMPILER_INITIALIZATION,
            BuildTimeMetricName::CodeAnalysis => BuildTime::CODE_ANALYSIS,
            BuildTimeMetricName::CodeGeneration => BuildTime::CODE_GENERATION,
            BuildTimeMetricName::ClasspathEntrySnapshotTransform => BuildTime::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM,
            BuildTimeMetricName::LoadClasses => BuildTime::LOAD_CLASSES,
            BuildTimeMetricName::SnapshotClasses => BuildTime::SNAPSHOT_CLASSES,
            BuildTimeMetricName::ReadClassesBasicInfo => BuildTime::READ_CLASSES_BASIC_INFO,
            BuildTimeMetricName::FindInaccessibleClasses => BuildTime::FIND_INACCESSIBLE_CLASSES,
            BuildTimeMetricName::SnapshotKotlinClasses => BuildTime::SNAPSHOT_KOTLIN_CLASSES,
            BuildTimeMetricName::SnapshotJavaClasses => BuildTime::SNAPSHOT_JAVA_CLASSES,
            BuildTimeMetricName::SaveClasspathEntrySnapshot => BuildTime::SAVE_CLASSPATH_ENTRY_SNAPSHOT,
        }
    }
}
