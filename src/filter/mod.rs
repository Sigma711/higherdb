pub mod bloom;

/// `FilterPolicy` is an algorithm for probabilistically encoding a set of keys.
/// The canonical implementation is a Bloom filter.
/// Every `FilterPolicy` has a name. This names the algorithm itself, not any one
/// particular instance. Aspects specific to a particular instance, such as the set
/// of keys or any other parameters, will be encoded in the byte filter returned by
/// `new_filter_writer`.
pub trait FilterPolicy: Send + Sync {
    fn name(&self) -> &str;

    /// `may_contain` returns whether the encoded filter may contain given key.
    /// False positives are possible, where it returns true for keys not in the
    /// original set.
    fn may_contain(&self, filter: &[u8], key: &[u8]) -> bool;

    /// Creates a filter based on given keys
    // TODO: use another type instead of &[Vec<u8>]
    fn create_filter(&self, keys: &[Vec<u8>]) -> Vec<u8>;
}
