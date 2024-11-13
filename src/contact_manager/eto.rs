use crate::generate_basic_volume_manager;

// With ETO the delay due to the queue is taken into account
// and the updates are not automatic, the queue is expected to be modified by
// external means
generate_basic_volume_manager!(ETOManager, true, false);
