//! Declarative framework for designing efficient causal inference algorithms.
//!
//! Provides a way to specify reachability-based algorithm with rule tables.
//! Best used through the wrapper packages ciflypy and ciflyr for Python and R.
//! Find more information on the [CIfly website](https://cifly.pages.dev).

mod expression;
pub mod instance;
pub mod reach;
pub mod ruletable;
pub use instance::{Graph, ParseGraphError, ParseSetsError, Sets};
pub use reach::Settings;
pub use ruletable::{ReadRuletableError, Ruletable};
