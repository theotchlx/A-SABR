use std::cmp::Ordering;
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{contact_manager::ContactManager, route_stage::RouteStage};

pub mod hop;
pub mod sabr;

/// A trait that allows RouteStages to define custom distance comparison strategies.
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
    fn cmp(first: &RouteStage<CM>, second: &RouteStage<CM>) -> Ordering;

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
    fn eq(first: &RouteStage<CM>, second: &RouteStage<CM>) -> bool;
}

/// A helper structure for providing ordering of `Rc<RefCell<RouteStage<CM>>>`
/// using custom `RouteStage<CM>` ordering defined by the trait `Distance<CM>`.
pub struct DistanceWrapper<CM: ContactManager, D: Distance<CM>>(
    pub Rc<RefCell<RouteStage<CM>>>,
    #[doc(hidden)] pub PhantomData<D>,
);

impl<CM: ContactManager, D: Distance<CM>> DistanceWrapper<CM, D> {
    pub fn new(route_stage: Rc<RefCell<RouteStage<CM>>>) -> Self {
        Self(route_stage, PhantomData)
    }
}

impl<CM: ContactManager, D: Distance<CM>> Ord for DistanceWrapper<CM, D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        D::cmp(&self.0.borrow(), &other.0.borrow())
    }
}

impl<CM: ContactManager, D: Distance<CM>> PartialOrd for DistanceWrapper<CM, D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<CM: ContactManager, D: Distance<CM>> PartialEq for DistanceWrapper<CM, D> {
    fn eq(&self, other: &Self) -> bool {
        D::eq(&self.0.borrow(), &other.0.borrow())
    }
}

impl<CM: ContactManager, D: Distance<CM>> Eq for DistanceWrapper<CM, D> {}
