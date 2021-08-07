use super::*;
use crate::core::base::{BaseSlot, BlockError, EntryContext, MetricEvent, StatNode, StatSlot};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

const STAT_SLOT_ORDER: u32 = 3000;

pub struct StandaloneStatSlot {}

lazy_static! {
    pub static ref DEFAULT_STAND_ALONE_STAT_SLOT: Arc<StandaloneStatSlot> =
        Arc::new(StandaloneStatSlot {});
}

pub fn default_stand_alone_stat_slot() -> Arc<StandaloneStatSlot> {
    DEFAULT_STAND_ALONE_STAT_SLOT.clone()
}

impl BaseSlot for StandaloneStatSlot {
    fn order(&self) -> u32 {
        STAT_SLOT_ORDER
    }
}

impl StatSlot for StandaloneStatSlot {
    fn on_entry_pass(&self, ctx: Rc<RefCell<EntryContext>>) {
        let ctx = ctx.borrow();
        let res = ctx.resource().name();
        let input = ctx.input();
        for tc in get_traffic_controller_list_for(res) {
            if !tc.bound_stat().reuse_global() {
                tc.bound_stat()
                    .write_only_metric()
                    .unwrap()
                    .add_count(MetricEvent::Pass, input.batch_count() as u64);
            }
        }
    }

    fn on_entry_blocked(&self, _ctx: Rc<RefCell<EntryContext>>, _block_error: Option<BlockError>) {}

    fn on_completed(&self, _ctx: Rc<RefCell<EntryContext>>) {}
}
