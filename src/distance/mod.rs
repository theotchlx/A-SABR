use std::cmp::Ordering;

use crate::{contact_manager::ContactManager, route_stage::RouteStage};

pub mod hop;
pub mod sabr;

/// A trait that allows RouteStages to befind custom distance comparison strategies.
///
/// # Type Parameters
/// - `CM`: A type that implements the `ContactManager` trait, representing the contact management
///         system used to manage and compare routes.
pub trait Distance<CM>
where
    Self: Sized,
    CM: ContactManager,
{
    /// Compares the distances between two `RouteStage` instances.
    ///
    /// This method provides a total ordering of `RouteStage` instances based on
    /// their distances, returning an `Ordering` (`Less`, `Equal`, or `Greater`)
    /// based on whether the `first` route is shorter, equal to, or longer than
    /// the `second` route.
    ///
    /// # Parameters
    /// - `first`: The first route stage to compare.
    /// - `second`: The second route stage to compare.
    ///
    /// # Returns
    /// - `Ordering::Less` if `first` is shorter than `second`.
    /// - `Ordering::Equal` if `first` and `second` are the same.
    /// - `Ordering::Greater` if `first` is longer than `second`.
    fn cmp(first: &RouteStage<CM, Self>, second: &RouteStage<CM, Self>) -> Ordering;

    /// Partially compares the distances between two `RouteStage` instances.
    ///
    /// This method provides a partial ordering of `RouteStage` instances based on
    /// their distances, returning an `Option<Ordering>` if a valid ordering exists.
    /// If the comparison cannot be performed (e.g., due to NaN values), `None` is returned.
    ///
    /// # Parameters
    /// - `first`: The first route stage to compare.
    /// - `second`: The second route stage to compare.
    ///
    /// # Returns
    /// - `Some(Ordering)` if a valid ordering can be determined.
    /// - `None` if the comparison is undefined.
    fn partial_cmp(first: &RouteStage<CM, Self>, second: &RouteStage<CM, Self>)
        -> Option<Ordering>;

    /// Checks if two `RouteStage` instances are equal in distance.
    ///
    /// This method determines if the distances of `first` and `second` are equal.
    ///
    /// # Parameters
    /// - `first`: The first route stage to check.
    /// - `second`: The second route stage to check.
    ///
    /// # Returns
    /// - `true` if `first` and `second` are equal in distance.
    /// - `false` otherwise.
    fn eq(first: &RouteStage<CM, Self>, second: &RouteStage<CM, Self>) -> bool;
}
