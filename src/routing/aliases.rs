use crate::{
    distance::{hop::Hop, sabr::SABR},
    pathfinding::{
        contact_graph::{ContactGraphPath, ContactGraphTree},
        first_depleted::FirstDepleted,
        first_ending::FirstEnding,
        mpt::{MptPath, MptTree},
        node_graph::{NodeGraphPath, NodeGraphTree},
    },
    route_storage::{cache::TreeCache, table::RoutingTable},
};

use super::{cgr::Cgr, spsn::Spsn};

type SpsnMpt<NM, CM> = Spsn<NM, CM, SABR, MptTree<NM, CM, SABR>, TreeCache<NM, CM, SABR>>;
type SpsnNodeGraph<NM, CM> =
    Spsn<NM, CM, SABR, NodeGraphTree<NM, CM, SABR>, TreeCache<NM, CM, SABR>>;
type SpsnContactGraph<NM, CM> =
    Spsn<NM, CM, SABR, ContactGraphTree<NM, CM, SABR>, TreeCache<NM, CM, SABR>>;
type CgrFirstEndingMpt<NM, CM> =
    Cgr<NM, CM, SABR, FirstEnding<NM, CM, SABR, MptPath<NM, CM, SABR>>, RoutingTable<NM, CM, SABR>>;
type CgrFirstDepletedMpt<NM, CM> = Cgr<
    NM,
    CM,
    SABR,
    FirstDepleted<NM, CM, SABR, MptPath<NM, CM, SABR>>,
    RoutingTable<NM, CM, SABR>,
>;
type CgrFirstEndingNodeGraph<NM, CM> = Cgr<
    NM,
    CM,
    SABR,
    FirstEnding<NM, CM, SABR, NodeGraphPath<NM, CM, SABR>>,
    RoutingTable<NM, CM, SABR>,
>;
type CgrFirstDepletedNodeGraph<NM, CM> = Cgr<
    NM,
    CM,
    SABR,
    FirstDepleted<NM, CM, SABR, NodeGraphPath<NM, CM, SABR>>,
    RoutingTable<NM, CM, SABR>,
>;
type CgrFirstEndingContactGraph<NM, CM> = Cgr<
    NM,
    CM,
    SABR,
    FirstEnding<NM, CM, SABR, ContactGraphPath<NM, CM, SABR>>,
    RoutingTable<NM, CM, SABR>,
>;
type CgrFirstDepletedContactGraph<NM, CM> = Cgr<
    NM,
    CM,
    SABR,
    FirstDepleted<NM, CM, SABR, ContactGraphPath<NM, CM, SABR>>,
    RoutingTable<NM, CM, SABR>,
>;

type SpsnHopMpt<NM, CM> = Spsn<NM, CM, Hop, MptTree<NM, CM, Hop>, TreeCache<NM, CM, Hop>>;
type SpsnHopNodeGraph<NM, CM> =
    Spsn<NM, CM, Hop, NodeGraphTree<NM, CM, Hop>, TreeCache<NM, CM, Hop>>;
type SpsnHopContactGraph<NM, CM> =
    Spsn<NM, CM, Hop, ContactGraphTree<NM, CM, Hop>, TreeCache<NM, CM, Hop>>;
type CgrHopFirstEndingMpt<NM, CM> =
    Cgr<NM, CM, Hop, FirstEnding<NM, CM, Hop, MptPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;
type CgrHopFirstDepletedMpt<NM, CM> =
    Cgr<NM, CM, Hop, FirstDepleted<NM, CM, Hop, MptPath<NM, CM, Hop>>, RoutingTable<NM, CM, Hop>>;
type CgrHopFirstEndingNodeGraph<NM, CM> = Cgr<
    NM,
    CM,
    Hop,
    FirstEnding<NM, CM, Hop, NodeGraphPath<NM, CM, Hop>>,
    RoutingTable<NM, CM, Hop>,
>;
type CgrHopFirstDepletedNodeGraph<NM, CM> = Cgr<
    NM,
    CM,
    Hop,
    FirstDepleted<NM, CM, Hop, NodeGraphPath<NM, CM, Hop>>,
    RoutingTable<NM, CM, Hop>,
>;
type CgrHopFirstEndingContactGraph<NM, CM> = Cgr<
    NM,
    CM,
    Hop,
    FirstEnding<NM, CM, Hop, ContactGraphPath<NM, CM, Hop>>,
    RoutingTable<NM, CM, Hop>,
>;
type CgrHopFirstDepletedContactGraph<NM, CM> = Cgr<
    NM,
    CM,
    Hop,
    FirstDepleted<NM, CM, Hop, ContactGraphPath<NM, CM, Hop>>,
    RoutingTable<NM, CM, Hop>,
>;
