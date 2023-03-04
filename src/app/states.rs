use yewdux::store::Store;

#[derive(Default, Debug, Clone, PartialEq, Store)]
pub(crate) struct CollisionState {
    pub(crate) on_collision_stay: bool,
    pub(crate) url: String,
}
