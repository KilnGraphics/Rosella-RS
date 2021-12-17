/// Utilities for globally unique identifiers.
///
/// A UUID is made up of 2 parts. A global id and a local id. The global id acts as a identifier for
/// local address spaces allowing systems to create their own generation methods for local ids while
/// retaining global uniqueness.

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::num::NonZeroU64;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

/// A global id backed by a 64 bit value.
///
/// Global ids are guaranteed to be globally unique. They are generated by an incrementing u64 bit
/// internal counter and will always be non zero.
///
/// # Examples
///
/// ```
/// use rosella_rs::util::id::GlobalId;
///
/// // Creates a new global id
/// let id = GlobalId::new();
///
/// // This is still the same id
/// let same_id = id.clone();
///
/// assert_eq!(id, same_id);
///
/// // Creates a new different global id
/// let other_id = GlobalId::new();
///
/// assert_ne!(id, other_id);
///
/// // 0 is a niche so Options are free
/// assert_eq!(8, std::mem::size_of::<Option<GlobalId>>());
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GlobalId(NonZeroU64);

// Need to reserve value 1 for the NamedId address space
static NEXT_GLOBAL_ID: AtomicU64 = AtomicU64::new(2u64);

impl GlobalId {
    /// Creates a new globally unique id
    ///
    /// # Panics
    ///
    /// This function will panic if the internal 64bit counter overflows.
    pub fn new() -> Self {
        let next = NEXT_GLOBAL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        match NonZeroU64::new(next) {
            Some(val) => Self(val),
            None => panic!("GlobalId overflow!")
        }
    }

    /// Creates a global id from a raw 64bit value.
    ///
    /// This value **must** have previously been created by a call to [`GlobalId::new()`] otherwise
    /// this **will** result in undefined behaviour.
    ///
    /// # Panics
    ///
    /// The function will panic if the id is `0`.
    pub const fn from_raw(id: u64) -> Self {
        if id == 0u64 {
            panic!("Id must not be 0");
        }

        unsafe { // Need const unwrap
            Self(NonZeroU64::new_unchecked(id))
        }
    }

    /// Returns the raw 64bit global id.
    pub fn get_raw(&self) -> u64 {
        self.0.get()
    }
}

impl Into<u64> for GlobalId {
    fn into(self) -> u64 {
        self.get_raw()
    }
}

impl Debug for GlobalId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GlobalId").field(&self.get_raw()).finish()
    }
}

/// A local id.
///
/// While global ids are guaranteed to be globally unique, local ids must not be and can be
/// generated in any way. The pair of a global id and a local id creates a globally unique
/// identifier.
///
/// Local ids are non zero u64 values.
///
/// # Examples
///
/// ```
/// use rosella_rs::util::id::LocalId;
///
/// // Creates a new local id with a value of 1
/// let id = LocalId::from_raw(1u64);
///
/// let same_id = id.clone();
/// assert_eq!(id, same_id);
///
/// // Local ids may not be globally unique
/// let still_same_id = LocalId::from_raw(1u64);
/// assert_eq!(id, still_same_id);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalId(NonZeroU64);

impl LocalId {
    /// Creates a local id for a raw value.
    ///
    /// The value must not be 0.
    pub const fn from_raw(value: u64) -> Self {
        if value == 0u64 {
            panic!("Local id must not be 0");
        }

        unsafe { // Need const unwrap
            Self(NonZeroU64::new_unchecked(value))
        }
    }

    /// Creates a local id from a hash value. if the hash is 0 it will be set to 1.
    pub const fn from_hash(mut hash: u64) -> Self {
        if hash == 0u64 {
            hash = 1u64;
        }

        unsafe { // Need const unwrap
            Self(NonZeroU64::new_unchecked(hash))
        }
    }

    pub const fn get_raw(&self) -> u64 {
        self.0.get()
    }
}

impl Debug for LocalId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LocalId").field(&self.get_raw()).finish()
    }
}

/// A universally unique identified.
///
/// A uuid is made up of a global id, local id pair.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UUID {
    pub global: GlobalId,
    pub local: LocalId,
}

/// A utility struct providing a simple incrementing counter local id generator.
///
/// The generator will create its own global id. Local ids will be generated from a incrementing
/// counter.
///
/// # Examples
/// ```
/// use rosella_rs::util::id::*;
///
/// // Create a new generator
/// let generator = IncrementingGenerator::new();
///
/// // A new uuid
/// let some_uuid = generator.next().unwrap();
///
/// // The global id of the generator will be used for uuids
/// assert_eq!(generator.get_global_id(), some_uuid.global);
///
/// // Some other uuid
/// let other_uuid = generator.next().unwrap();
/// assert_ne!(some_uuid, other_uuid);
/// ```
pub struct IncrementingGenerator {
    global: GlobalId,
    next: AtomicU64,
}

impl IncrementingGenerator {
    /// Creates a new generator with a new global id and a local id starting at 0.
    pub fn new() -> Self {
        Self {
            global: GlobalId::new(),
            next: AtomicU64::new(1),
        }
    }

    /// Returns the global id of the generator.
    pub fn get_global_id(&self) -> GlobalId {
        self.global
    }

    /// Creates a new uuid
    pub fn next(&self) -> Option<UUID> {
        let local = self.next.fetch_add(1u64, std::sync::atomic::Ordering::Relaxed);

        Some(UUID {
            global: self.global,
            local: LocalId::from_raw(local),
        })
    }
}

/// A UUID generated from a string.
///
/// NamedUUIDs use a predefined global id with the local id being calculated as the hash of a
/// string. The name is stored along side the UUID for easy debugging or printing. The name is
/// stored by Arc enabling fast Copying of the struct.
#[derive(Clone, Debug)]
pub struct NamedUUID {
    name: Arc<String>,
    id: LocalId,
}

impl NamedUUID {
    /// The global id used by all NamedUUIDs
    pub const GLOBAL_ID: GlobalId = GlobalId::from_raw(1u64);

    const fn hash_str_const(name: &str) -> u64 {
        xxhash_rust::const_xxh3::xxh3_64(name.as_bytes())
    }

    fn hash_str(name: &str) -> u64 {
        xxhash_rust::xxh3::xxh3_64(name.as_bytes())
    }

    pub const fn new_const(name: &str) -> UUID {
        let hash = Self::hash_str_const(name);

        UUID { global: Self::GLOBAL_ID, local: LocalId::from_hash(hash) }
    }

    pub fn new(name: String) -> NamedUUID {
        let hash = Self::hash_str(name.as_str());

        NamedUUID { name: Arc::new(name), id: LocalId::from_hash(hash) }
    }

    /// Returns the string that generated the UUID
    pub fn get_name(&self) -> &String {
        self.name.as_ref()
    }

    /// Returns the uuid
    pub fn get_uuid(&self) -> UUID {
        UUID {
            global: Self::GLOBAL_ID,
            local: self.id,
        }
    }

    /// Returns the global id
    pub fn get_global_id(&self) -> GlobalId {
        Self::GLOBAL_ID
    }

    /// Returns the local id
    pub fn get_local_id(&self) -> LocalId {
        self.id
    }
}

impl PartialEq for NamedUUID {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for NamedUUID {
}

impl PartialEq<UUID> for NamedUUID {
    fn eq(&self, other: &UUID) -> bool {
        self.get_uuid().eq(other)
    }
}

impl PartialOrd for NamedUUID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for NamedUUID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd<UUID> for NamedUUID {
    fn partial_cmp(&self, other: &UUID) -> Option<Ordering> {
        self.get_uuid().partial_cmp(other)
    }
}

impl Hash for NamedUUID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // The hash should be identical to the one generated from the uuid
        self.get_uuid().hash(state)
    }
}

impl Into<UUID> for NamedUUID {
    fn into(self) -> UUID {
        self.get_uuid()
    }
}
