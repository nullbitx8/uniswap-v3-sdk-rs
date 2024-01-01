use super::{Tick, TickTrait};
use anyhow::Result;
use thiserror::Error;

/// Provides information about ticks
pub trait TickDataProvider<T: TickTrait> {
    /// Return information corresponding to a specific tick
    ///
    /// # Arguments
    ///
    /// * `tick`: The tick to load
    ///
    /// returns: Result<impl TickTrait+Sized, Error>
    ///
    fn get_tick(&self, tick: i32) -> Result<&T>;

    /// Return the next tick that is initialized within a single word
    ///
    /// # Arguments
    ///
    /// * `tick`: The current tick
    /// * `lte`: Whether the next tick should be lte the current tick
    /// * `tick_spacing`: The tick spacing of the pool
    ///
    /// returns: Result<(i32, bool), Error>
    ///
    fn next_initialized_tick_within_one_word(
        &self,
        tick: i32,
        lte: bool,
        tick_spacing: i32,
    ) -> Result<(i32, bool)>;
}

#[derive(Error, Debug)]
#[error("No tick data provider was given")]
pub struct NoTickDataError;

/// This tick data provider does not know how to fetch any tick data. It throws whenever it is required.
/// Useful if you do not need to load tick data for your use case.
pub struct NoTickDataProvider;

impl TickDataProvider<Tick> for NoTickDataProvider {
    fn get_tick(&self, _: i32) -> Result<&Tick> {
        Err(NoTickDataError.into())
    }

    fn next_initialized_tick_within_one_word(
        &self,
        _: i32,
        _: bool,
        _: i32,
    ) -> Result<(i32, bool)> {
        Err(NoTickDataError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_tick_data_provider() {
        let tick_data_provider = NoTickDataProvider;
        assert_eq!(
            tick_data_provider.get_tick(0).unwrap_err().to_string(),
            NoTickDataError.to_string()
        );
        assert_eq!(
            tick_data_provider
                .next_initialized_tick_within_one_word(0, false, 1)
                .unwrap_err()
                .to_string(),
            NoTickDataError.to_string()
        );
    }
}
