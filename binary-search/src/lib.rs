fn _find<ElemType>(array: &[(usize, ElemType)], key: ElemType) -> Option<usize>
where
    ElemType: Ord,
{
    if array.len() == 1 {
        if array[0].1 == key {
            return Some(array[0].0);
        } else {
            return None;
        }
    }

    let (chunk1, chunk2) = array.split_at(array.len() / 2);
    if let Some(x) = chunk1.last() {
        if x.1 >= key {
            return _find(chunk1, key);
        }
    }
    if let Some(x) = chunk2.first() {
        if x.1 <= key {
            return _find(chunk2, key);
        }
    }

    None
}

pub fn find<ContainerType, ElemType>(array: ContainerType, key: ElemType) -> Option<usize>
where
    ContainerType: AsRef<[ElemType]>,
    ElemType: Ord,
{
    _find(&array.as_ref().iter().enumerate().collect::<Vec<_>>()[..], &key)
}
