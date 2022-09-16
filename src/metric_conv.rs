use crate::metric::{BuildPerformance, BuildTime};
use crate::metric_name;
use std::convert::From;

impl From<metric_name::BuildPerformance> for BuildPerformance<'static> {
    fn from(name: metric_name::BuildPerformance) -> Self {
        match name {
            metric_name::BuildPerformance::CacheDirectorySize => {
                BuildPerformance::CACHE_DIRECTORY_SIZE
            }
            metric_name::BuildPerformance::LookupSize => BuildPerformance::LOOKUP_SIZE,
            metric_name::BuildPerformance::SnapshotSize => BuildPerformance::SNAPSHOT_SIZE,
            metric_name::BuildPerformance::BundleSize => BuildPerformance::BUNDLE_SIZE,
            metric_name::BuildPerformance::CompileIteration => BuildPerformance::COMPILE_ITERATION,
            metric_name::BuildPerformance::ClasspathEntrySnapshotTransformExecutionCount => {
                BuildPerformance::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM_EXECUTION_COUNT
            }
            metric_name::BuildPerformance::JarClasspathEntrySize => {
                BuildPerformance::JAR_CLASSPATH_ENTRY_SIZE
            }
            metric_name::BuildPerformance::JarClasspathEntrySnapshotSize => {
                BuildPerformance::JAR_CLASSPATH_ENTRY_SNAPSHOT_SIZE
            }
            metric_name::BuildPerformance::DirectoryClasspathEntrySnapshotSize => {
                BuildPerformance::DIRECTORY_CLASSPATH_ENTRY_SNAPSHOT_SIZE
            }
            metric_name::BuildPerformance::ComputeClasspathChangesExecutionCount => {
                BuildPerformance::COMPUTE_CLASSPATH_CHANGES_EXECUTION_COUNT
            }
            metric_name::BuildPerformance::ShrinkAndSaveClasspathSnapshotExecutionCount => {
                BuildPerformance::SHRINK_AND_SAVE_CLASSPATH_SNAPSHOT_EXECUTION_COUNT
            }
            metric_name::BuildPerformance::ClasspathEntryCount => {
                BuildPerformance::CLASSPATH_ENTRY_COUNT
            }
            metric_name::BuildPerformance::ClasspathSnapshotSize => {
                BuildPerformance::CLASSPATH_SNAPSHOT_SIZE
            }
            metric_name::BuildPerformance::ShrunkClasspathSnapshotSize => {
                BuildPerformance::SHRUNK_CLASSPATH_SNAPSHOT_SIZE
            }
            metric_name::BuildPerformance::LoadClasspathSnapshotExecutionCount => {
                BuildPerformance::LOAD_CLASSPATH_SNAPSHOT_EXECUTION_COUNT
            }
            metric_name::BuildPerformance::LoadClasspathEntrySnapshotCacheHits => {
                BuildPerformance::LOAD_CLASSPATH_ENTRY_SNAPSHOT_CACHE_HITS
            }
            metric_name::BuildPerformance::LoadClasspathEntrySnapshotCacheMisses => {
                BuildPerformance::LOAD_CLASSPATH_ENTRY_SNAPSHOT_CACHE_MISSES
            }
        }
    }
}

impl From<metric_name::BuildTime> for BuildTime<'static> {
    fn from(name: metric_name::BuildTime) -> Self {
        match name {
            metric_name::BuildTime::GradleTask => BuildTime::GRADLE_TASK,
            metric_name::BuildTime::GradleTaskAction => BuildTime::GRADLE_TASK_ACTION,
            metric_name::BuildTime::ClearOutput => BuildTime::CLEAR_OUTPUT,
            metric_name::BuildTime::BackupOutput => BuildTime::BACKUP_OUTPUT,
            metric_name::BuildTime::RestoreOutputFromBackup => BuildTime::RESTORE_OUTPUT_FROM_BACKUP,
            metric_name::BuildTime::ConnectToDaemon => BuildTime::CONNECT_TO_DAEMON,
            metric_name::BuildTime::ClearJarCache => BuildTime::CLEAR_JAR_CACHE,
            metric_name::BuildTime::CalculateOutputSize => BuildTime::CALCULATE_OUTPUT_SIZE,
            metric_name::BuildTime::RunCompilation => BuildTime::RUN_COMPILATION,
            metric_name::BuildTime::NonIncrementalCompilationInProcess => BuildTime::NON_INCREMENTAL_COMPILATION_IN_PROCESS,
            metric_name::BuildTime::NonIncrementalCompilationOutOfProcess => BuildTime::NON_INCREMENTAL_COMPILATION_OUT_OF_PROCESS,
            metric_name::BuildTime::NonIncrementalCompilationDaemon => BuildTime::NON_INCREMENTAL_COMPILATION_DAEMON,
            metric_name::BuildTime::IncrementalCompilationDaemon => BuildTime::INCREMENTAL_COMPILATION_DAEMON,
            metric_name::BuildTime::StoreBuildInfo => BuildTime::STORE_BUILD_INFO,
            metric_name::BuildTime::JarSnapshot => BuildTime::JAR_SNAPSHOT,
            metric_name::BuildTime::SetUpAbiSnapshots => BuildTime::SET_UP_ABI_SNAPSHOTS,
            metric_name::BuildTime::IcAnalyzeJarFiles => BuildTime::IC_ANALYZE_JAR_FILES,
            metric_name::BuildTime::IcCalculateInitialDirtySet => BuildTime::IC_CALCULATE_INITIAL_DIRTY_SET,
            metric_name::BuildTime::ComputeClasspathChanges => BuildTime::COMPUTE_CLASSPATH_CHANGES,
            metric_name::BuildTime::LoadCurrentClasspathSnapshot => BuildTime::LOAD_CURRENT_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::RemoveDuplicateClasses => BuildTime::REMOVE_DUPLICATE_CLASSES,
            metric_name::BuildTime::ShrinkCurrentClasspathSnapshot => BuildTime::SHRINK_CURRENT_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::GetLookupSymbols => BuildTime::GET_LOOKUP_SYMBOLS,
            metric_name::BuildTime::FindReferencedClasses => BuildTime::FIND_REFERENCED_CLASSES,
            metric_name::BuildTime::FindTransitivelyReferencedClasses => BuildTime::FIND_TRANSITIVELY_REFERENCED_CLASSES,
            metric_name::BuildTime::LoadShrunkPreviousClasspathSnapshot => BuildTime::LOAD_SHRUNK_PREVIOUS_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::ComputeChangedAndImpactedSet => BuildTime::COMPUTE_CHANGED_AND_IMPACTED_SET,
            metric_name::BuildTime::ComputeClassChanges => BuildTime::COMPUTE_CLASS_CHANGES,
            metric_name::BuildTime::ComputeKotlinClassChanges => BuildTime::COMPUTE_KOTLIN_CLASS_CHANGES,
            metric_name::BuildTime::ComputeJavaClassChanges => BuildTime::COMPUTE_JAVA_CLASS_CHANGES,
            metric_name::BuildTime::ComputeImpactedSet => BuildTime::COMPUTE_IMPACTED_SET,
            metric_name::BuildTime::IcAnalyzeChangesInDependencies => BuildTime::IC_ANALYZE_CHANGES_IN_DEPENDENCIES,
            metric_name::BuildTime::IcFindHistoryFiles => BuildTime::IC_FIND_HISTORY_FILES,
            metric_name::BuildTime::IcAnalyzeHistoryFiles => BuildTime::IC_ANALYZE_HISTORY_FILES,
            metric_name::BuildTime::IcAnalyzeChangesInJavaSources => BuildTime::IC_ANALYZE_CHANGES_IN_JAVA_SOURCES,
            metric_name::BuildTime::IcAnalyzeChangesInAndroidLayouts => BuildTime::IC_ANALYZE_CHANGES_IN_ANDROID_LAYOUTS,
            metric_name::BuildTime::IcDetectRemovedClasses => BuildTime::IC_DETECT_REMOVED_CLASSES,
            metric_name::BuildTime::ClearOutputOnRebuild => BuildTime::CLEAR_OUTPUT_ON_REBUILD,
            metric_name::BuildTime::IcUpdateCaches => BuildTime::IC_UPDATE_CACHES,
            metric_name::BuildTime::CompilationRound => BuildTime::COMPILATION_ROUND,
            metric_name::BuildTime::IcWriteHistoryFile => BuildTime::IC_WRITE_HISTORY_FILE,
            metric_name::BuildTime::ShrinkAndSaveCurrentClasspathSnapshotAfterCompilation => BuildTime::SHRINK_AND_SAVE_CURRENT_CLASSPATH_SNAPSHOT_AFTER_COMPILATION,
            metric_name::BuildTime::IncrementalShrinkCurrentClasspathSnapshot => BuildTime::INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::IncrementalLoadCurrentClasspathSnapshot => BuildTime::INCREMENTAL_LOAD_CURRENT_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::IncrementalLoadShrunkCurrentClasspathSnapshotAgainstPreviousLookups => BuildTime::INCREMENTAL_LOAD_SHRUNK_CURRENT_CLASSPATH_SNAPSHOT_AGAINST_PREVIOUS_LOOKUPS,
            metric_name::BuildTime::NonIncrementalShrinkCurrentClasspathSnapshot => BuildTime::NON_INCREMENTAL_SHRINK_CURRENT_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::NonIncrementalLoadCurrentClasspathSnapshot => BuildTime::NON_INCREMENTAL_LOAD_CURRENT_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::SaveShrunkCurrentClasspathSnapshot => BuildTime::SAVE_SHRUNK_CURRENT_CLASSPATH_SNAPSHOT,
            metric_name::BuildTime::CompilerPerformance => BuildTime::COMPILER_PERFORMANCE,
            metric_name::BuildTime::CompilerInitialization => BuildTime::COMPILER_INITIALIZATION,
            metric_name::BuildTime::CodeAnalysis => BuildTime::CODE_ANALYSIS,
            metric_name::BuildTime::CodeGeneration => BuildTime::CODE_GENERATION,
            metric_name::BuildTime::ClasspathEntrySnapshotTransform => BuildTime::CLASSPATH_ENTRY_SNAPSHOT_TRANSFORM,
            metric_name::BuildTime::LoadClasses => BuildTime::LOAD_CLASSES,
            metric_name::BuildTime::SnapshotClasses => BuildTime::SNAPSHOT_CLASSES,
            metric_name::BuildTime::ReadClassesBasicInfo => BuildTime::READ_CLASSES_BASIC_INFO,
            metric_name::BuildTime::FindInaccessibleClasses => BuildTime::FIND_INACCESSIBLE_CLASSES,
            metric_name::BuildTime::SnapshotKotlinClasses => BuildTime::SNAPSHOT_KOTLIN_CLASSES,
            metric_name::BuildTime::SnapshotJavaClasses => BuildTime::SNAPSHOT_JAVA_CLASSES,
            metric_name::BuildTime::SaveClasspathEntrySnapshot => BuildTime::SAVE_CLASSPATH_ENTRY_SNAPSHOT,
        }
    }
}
