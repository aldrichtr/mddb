use nanoid::nanoid;
use uuid::Uuid;

// TODO: This needs to be configurable
const ALPHABET : [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
];

#[derive(Debug, PartialEq)]
pub enum IdType {
    Nanoid,
    Uuid,
}

#[derive(Debug)]
pub struct Id {
    pub kind : IdType,
    pub length : usize,
    _id : String,
}

impl Default for Id {
    fn default() -> Self {
        Self {
            kind: IdType::Nanoid,
            length: 23,
            _id: nanoid!(23, &ALPHABET)
        }
    }
}

impl Id {
    pub fn new(kind : Option<IdType>, len : Option<usize>) -> Self {
        // TODO: get these "defaults" from a config object
        let id_type = kind.unwrap_or(IdType::Nanoid);
        let mut id_length = len.unwrap_or(23);
        let mut _id = String::new();
        match id_type {
            IdType::Nanoid => {
                _id = nanoid!(id_length, &ALPHABET);
            }
            IdType::Uuid => {
                _id = Uuid::new_v4().to_string();
                id_length = _id.len();
            }
        };

        Self {
            kind : id_type,
            length : id_length,
            _id : _id,
        }
    }

    pub fn to_string(&self) -> &String {
        let st = &self._id;
        st
    }
}

#[cfg(test)]

mod tests {
    use super::Id;
    use super::IdType::{Nanoid, Uuid};

    #[test]
    fn test_new_id_with_default_id() {
        let id = Id::new(None, None);
        assert_eq!(23, id.length);
        assert_eq!(Nanoid, id.kind);
        assert_eq!(23, id.to_string().len());
    }

    #[test]
    fn test_new_id_with_uuid_id() {
        let id = Id::new(Some(Uuid), None);
        assert_eq!(36, id.length);
        assert_eq!(Uuid, id.kind);
        assert_eq!(36, id.to_string().len());
    }

    #[test]
    fn test_new_id_with_nanoid_id() {
        let id = Id::new(Some(Nanoid), Some(40));
        assert_eq!(40, id.length);
        assert_eq!(Nanoid, id.kind);
        assert_eq!(40, id.to_string().len());
    }
}
