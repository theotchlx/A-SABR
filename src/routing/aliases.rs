use crate::{
    distance::{hop::Hop, sabr::SABR},
    pathfinding::{
        mpt::{MptPath, MptTree},
        node_graph::{NodeGraphPath, NodeGraphTree},
    },
    route_storage::{cache::TreeCache, table::RoutingTable},
};

#[cfg(feature = "contact_work_area")]
use crate::pathfinding::contact_graph::{ContactGraphPath, ContactGraphTree};
#[cfg(all(feature = "contact_suppression", feature = "first_depleted"))]
use crate::pathfinding::first_depleted::FirstDepleted;
#[cfg(feature = "contact_suppression")]
use crate::pathfinding::first_ending::FirstEnding;

use super::{cgr::Cgr, spsn::Spsn};

pub type SpsnMpt<NM, CM> = Spsn<NM, CM, MptTree<NM, CM, SABR>, TreeCache<NM, CM>>;

pub type SpsnNodeGraph<NM, CM> = Spsn<NM, CM, NodeGraphTree<NM, CM, SABR>, TreeCache<NM, CM>>;

#[cfg(feature = "contact_work_area")]
pub type SpsnContactGraph<NM, CM> = Spsn<NM, CM, ContactGraphTree<NM, CM, SABR>, TreeCache<NM, CM>>;

#[cfg(feature = "contact_suppression")]
pub type CgrFirstEndingMpt<NM, CM> =
    Cgr<NM, CM, FirstEnding<NM, CM, MptPath<NM, CM, SABR>>, RoutingTable<NM, CM, SABR>>;

#[cfg(all(feature = "contact_suppression", feature = "first_depleted"))]
pub type CgrFirstDepletedMpt<NM, CM> =
    Cgr<NM, CM, FirstDepleted<NM, CM, MptPath<NM, CM, SABR>>, RoutingTable<NM, CM, SABR>>;

#[cfg(feature = "contact_suppression")]
pub type CgrFirstEndingNodeGraph<NM, CM> =
    Cgr<NM, CM, FirstEnding<NM, CM, NodeGraphPath<NM, CM, SABR>>, RoutingTable<NM, CM, SABR>>;

#[cfg(all(feature = "contact_suppression", feature = "first_depleted"))]
pub type CgrFirstDepletedNodeGraph<NM, CM> =
    Cgr<NM, CM, FirstDepleted<NM, CM, NodeGraphPath<NM, CM, SABR>>, RoutingTable<NM, CM, SABR>>;

#[cfg(all(feature = "contact_work_area", feature = "contact_suppression"))]
pub type CgrFirstEndingContactGraph<NM, CM> =
    Cgr<NM, CM, FirstEnding<NM, CM, ContactGraphPath<NM, CM, SABR>>, RoutingTable<NM, CM, SABR>>;

#[cfg(all(
    feature = "contact_work_area",
    feature = "contact_suppression",
    feature = "first_depleted"
))]
pub type CgrFirstDepletedContactGraph<NM, CM> =
    Cgr<NM, CM, FirstDepleted<NM, CM, ContactGraphPath<NM, CM, SABR>>, RoutingTable<NM, CM, SABR>>;

pub type SpsnHopMpt<NM, CM> = Spsn<NM, CM, MptTree<NM, CM, Hop>, TreeCache<NM, CM>>;

pub type SpsnHopNodeGraph<NM, CM> = Spsn<NM, CM, NodeGraphTree<NM, CM, Hop>, TreeCache<NM, CM>>;

#[cfg(feature = "contact_work_area")]
pub type SpsnHopContactGraph<NM, CM> =
    Spsn<NM, CM, ContactGraphTree<NM, CM, Hop>, TreeCache<NM, CM>>;

#[cfg(feature = "contact_suppression")]
pub type CgrHopFirstEndingMpt<NM, CM> =
    Cgr<NM, CM, FirstEnding<NM, CM, MptPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;

#[cfg(all(feature = "contact_suppression", feature = "first_depleted"))]
pub type CgrHopFirstDepletedMpt<NM, CM> =
    Cgr<NM, CM, FirstDepleted<NM, CM, MptPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;

#[cfg(feature = "contact_suppression")]
pub type CgrHopFirstEndingNodeGraph<NM, CM> =
    Cgr<NM, CM, FirstEnding<NM, CM, NodeGraphPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;

#[cfg(all(feature = "contact_suppression", feature = "first_depleted"))]
pub type CgrHopFirstDepletedNodeGraph<NM, CM> =
    Cgr<NM, CM, FirstDepleted<NM, CM, NodeGraphPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;

#[cfg(all(feature = "contact_work_area", feature = "contact_suppression"))]
pub type CgrHopFirstEndingContactGraph<NM, CM> =
    Cgr<NM, CM, FirstEnding<NM, CM, ContactGraphPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;

#[cfg(all(
    feature = "contact_work_area",
    feature = "contact_suppression",
    feature = "first_depleted"
))]
pub type CgrHopFirstDepletedContactGraph<NM, CM> =
    Cgr<NM, CM, FirstDepleted<NM, CM, ContactGraphPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;
