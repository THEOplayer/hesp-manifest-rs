pub fn normalize<'a, T, I>(parent: &'a mut Option<T>, mut children: I)
where
    T: std::cmp::PartialEq,
    I: Iterator<Item = &'a mut Option<T>>,
{
    if let Some(value @ Some(_)) = children.next() {
        let mut values = Vec::new();
        for child in children {
            if child != value {
                return;
            }
            values.push(child);
        }
        *parent = value.take();
        for child in values {
            *child = None;
        }
    }
}

#[macro_export]
macro_rules! normalize_tracks {
    ($self:expr, $value:ident) => (
        crate::util::normalize(
            &mut $self.$value,
            $self.tracks.iter_mut().map(|track| &mut track.$value),
        );
    );
    ($self:expr, $value:ident, $($others:ident),+) => (
        normalize_tracks!($self, $value);
        normalize_tracks!($self, $($others),+);
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_moves_value_up() {
        let mut parent = None;
        let mut children = vec![Some(1), Some(1), Some(1)];

        normalize(&mut parent, children.iter_mut());

        assert_eq!(parent, Some(1));
        assert_eq!(children, vec![None, None, None]);
    }

    #[test]
    fn normalize_does_nothing_on_empty_list() {
        let mut parent: Option<u8> = None;
        let mut children = Vec::new();

        normalize(&mut parent, children.iter_mut());

        assert_eq!(parent, None);
        assert_eq!(children, Vec::new());
    }

    #[test]
    fn normalize_only_works_when_all_are_same() {
        let mut parent = None;
        let mut children = vec![Some(1), Some(1), Some(2), Some(1)];

        normalize(&mut parent, children.iter_mut());

        assert_eq!(parent, None);
        assert_eq!(children, vec![Some(1), Some(1), Some(2), Some(1)]);
    }
}
