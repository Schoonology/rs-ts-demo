///
/// If desired, this is a single touchpoint to replace anyhow with, say,
/// thiserror or some other, custom Error type. Ensure the exported Result
/// uses the desired Error.
///

/**
 * App-internal type for all Results.
 */
pub(crate) type Result<T> = anyhow::Result<T>;
