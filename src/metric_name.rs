/// The build reports includes metrics for build times per task type
/// as well as measurements of specific things relevant to build
/// performance. The Kotlin representation of these types are
/// data-carrying enums sent as a Map<Enum.name, Number> which
/// is hard to do with just regular Rust enums. To workaround
/// this limitation, we define the metric names as a bare enum
/// such that the Serde type for these can be `HashMap<MetricName, i64>`
/// and be deserialized easily. We provide From/To implementations
/// to convert these plain names into the fleshed out types that
/// actually match the Kotlin representation.
use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildTime {
    GradleTask,
    GradleTaskAction,
    ClearOutput,
    BackupOutput,
    RestoreOutputFromBackup,
    ConnectToDaemon,
    ClearJarCache,
    CalculateOutputSize,
    RunCompilation,
    NonIncrementalCompilationInProcess,
    NonIncrementalCompilationOutOfProcess,
    NonIncrementalCompilationDaemon,
    IncrementalCompilationDaemon,
    StoreBuildInfo,
    JarSnapshot,
    SetUpAbiSnapshots,
    IcAnalyzeJarFiles,
    IcCalculateInitialDirtySet,
    ComputeClasspathChanges,
    LoadCurrentClasspathSnapshot,
    RemoveDuplicateClasses,
    ShrinkCurrentClasspathSnapshot,
    GetLookupSymbols,
    FindReferencedClasses,
    FindTransitivelyReferencedClasses,
    LoadShrunkPreviousClasspathSnapshot,
    ComputeChangedAndImpactedSet,
    ComputeClassChanges,
    ComputeKotlinClassChanges,
    ComputeJavaClassChanges,
    ComputeImpactedSet,
    IcAnalyzeChangesInDependencies,
    IcFindHistoryFiles,
    IcAnalyzeHistoryFiles,
    IcAnalyzeChangesInJavaSources,
    IcAnalyzeChangesInAndroidLayouts,
    IcDetectRemovedClasses,
    ClearOutputOnRebuild,
    IcUpdateCaches,
    CompilationRound,
    IcWriteHistoryFile,
    ShrinkAndSaveCurrentClasspathSnapshotAfterCompilation,
    IncrementalShrinkCurrentClasspathSnapshot,
    IncrementalLoadCurrentClasspathSnapshot,
    IncrementalLoadShrunkCurrentClasspathSnapshotAgainstPreviousLookups,
    NonIncrementalShrinkCurrentClasspathSnapshot,
    NonIncrementalLoadCurrentClasspathSnapshot,
    SaveShrunkCurrentClasspathSnapshot,
    CompilerPerformance,
    CompilerInitialization,
    CodeAnalysis,
    CodeGeneration,
    ClasspathEntrySnapshotTransform,
    LoadClasses,
    SnapshotClasses,
    ReadClassesBasicInfo,
    FindInaccessibleClasses,
    SnapshotKotlinClasses,
    SnapshotJavaClasses,
    SaveClasspathEntrySnapshot,
}

#[derive(Eq, Hash, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildPerformance {
    CacheDirectorySize,
    LookupSize,
    SnapshotSize,
    BundleSize,
    CompileIteration,
    ClasspathEntrySnapshotTransformExecutionCount,
    JarClasspathEntrySize,
    JarClasspathEntrySnapshotSize,
    DirectoryClasspathEntrySnapshotSize,
    ComputeClasspathChangesExecutionCount,
    ShrinkAndSaveClasspathSnapshotExecutionCount,
    ClasspathEntryCount,
    ClasspathSnapshotSize,
    ShrunkClasspathSnapshotSize,
    LoadClasspathSnapshotExecutionCount,
    LoadClasspathEntrySnapshotCacheHits,
    LoadClasspathEntrySnapshotCacheMisses,
}
