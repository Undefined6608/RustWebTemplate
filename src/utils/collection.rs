use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::cmp::Ord;

/// 集合工具结构体
pub struct CollectionUtils;

impl CollectionUtils {
    /// 检查数组是否为空
    pub fn is_empty<T>(arr: &[T]) -> bool {
        arr.is_empty()
    }

    /// 检查数组是否不为空
    pub fn is_not_empty<T>(arr: &[T]) -> bool {
        !arr.is_empty()
    }

    /// 获取数组的第一个元素
    pub fn first<T>(arr: &[T]) -> Option<&T> {
        arr.first()
    }

    /// 获取数组的最后一个元素
    pub fn last<T>(arr: &[T]) -> Option<&T> {
        arr.last()
    }

    /// 安全地获取数组中指定索引的元素
    pub fn get<T>(arr: &[T], index: usize) -> Option<&T> {
        arr.get(index)
    }

    /// 数组去重
    pub fn unique<T: Clone + Hash + Eq>(arr: &[T]) -> Vec<T> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        
        for item in arr {
            if seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }
        
        result
    }

    /// 数组排序去重
    pub fn unique_sorted<T: Clone + Ord>(arr: &[T]) -> Vec<T> {
        let mut result: Vec<T> = arr.to_vec();
        result.sort();
        result.dedup();
        result
    }

    /// 分块处理数组
    pub fn chunk<T: Clone>(arr: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 {
            return vec![];
        }
        
        arr.chunks(size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// 数组分组
    pub fn group_by<T, K, F>(arr: &[T], key_fn: F) -> HashMap<K, Vec<T>>
    where
        T: Clone,
        K: Hash + Eq,
        F: Fn(&T) -> K,
    {
        let mut groups = HashMap::new();
        
        for item in arr {
            let key = key_fn(item);
            groups.entry(key).or_insert_with(Vec::new).push(item.clone());
        }
        
        groups
    }

    /// 数组求交集
    pub fn intersection<T: Clone + Hash + Eq>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let set1: HashSet<_> = arr1.iter().collect();
        let set2: HashSet<_> = arr2.iter().collect();
        
        set1.intersection(&set2)
            .map(|&item| item.clone())
            .collect()
    }

    /// 数组求并集
    pub fn union<T: Clone + Hash + Eq>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let mut set = HashSet::new();
        
        for item in arr1.iter().chain(arr2.iter()) {
            set.insert(item.clone());
        }
        
        set.into_iter().collect()
    }

    /// 数组求差集（arr1 - arr2）
    pub fn difference<T: Clone + Hash + Eq>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let set2: HashSet<_> = arr2.iter().collect();
        
        arr1.iter()
            .filter(|item| !set2.contains(item))
            .cloned()
            .collect()
    }

    /// 数组对称差集
    pub fn symmetric_difference<T: Clone + Hash + Eq>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let set1: HashSet<_> = arr1.iter().collect();
        let set2: HashSet<_> = arr2.iter().collect();
        
        set1.symmetric_difference(&set2)
            .map(|&item| item.clone())
            .collect()
    }

    /// 数组乱序
    pub fn shuffle<T>(arr: &mut [T]) {
        use rand::seq::SliceRandom;
        arr.shuffle(&mut rand::thread_rng());
    }

    /// 随机选择元素
    pub fn sample<T: Clone>(arr: &[T], count: usize) -> Vec<T> {
        use rand::seq::SliceRandom;
        
        if count >= arr.len() {
            return arr.to_vec();
        }
        
        arr.choose_multiple(&mut rand::thread_rng(), count)
            .cloned()
            .collect()
    }

    /// 查找元素的所有索引
    pub fn find_all_indices<T: PartialEq>(arr: &[T], target: &T) -> Vec<usize> {
        arr.iter()
            .enumerate()
            .filter_map(|(i, item)| if item == target { Some(i) } else { None })
            .collect()
    }

    /// 数组拉平（一层）
    pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
        nested.iter().flat_map(|v| v.iter()).cloned().collect()
    }

    /// 数组分区
    pub fn partition<T: Clone, P>(arr: &[T], predicate: P) -> (Vec<T>, Vec<T>)
    where
        P: Fn(&T) -> bool,
    {
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        
        for item in arr {
            if predicate(item) {
                true_items.push(item.clone());
            } else {
                false_items.push(item.clone());
            }
        }
        
        (true_items, false_items)
    }

    /// 计算数组的笛卡尔积
    pub fn cartesian_product<T: Clone>(arr1: &[T], arr2: &[T]) -> Vec<(T, T)> {
        let mut result = Vec::new();
        
        for item1 in arr1 {
            for item2 in arr2 {
                result.push((item1.clone(), item2.clone()));
            }
        }
        
        result
    }

    /// 数组旋转（左旋）
    pub fn rotate_left<T: Clone>(arr: &[T], positions: usize) -> Vec<T> {
        if arr.is_empty() {
            return Vec::new();
        }
        
        let len = arr.len();
        let positions = positions % len;
        
        let mut result = Vec::with_capacity(len);
        result.extend_from_slice(&arr[positions..]);
        result.extend_from_slice(&arr[..positions]);
        
        result
    }

    /// 数组旋转（右旋）
    pub fn rotate_right<T: Clone>(arr: &[T], positions: usize) -> Vec<T> {
        if arr.is_empty() {
            return Vec::new();
        }
        
        let len = arr.len();
        let positions = positions % len;
        
        Self::rotate_left(arr, len - positions)
    }

    /// 数组滑动窗口
    pub fn sliding_window<T: Clone>(arr: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 || size > arr.len() {
            return vec![];
        }
        
        arr.windows(size)
            .map(|window| window.to_vec())
            .collect()
    }

    /// 数组压缩（zip）
    pub fn zip<T: Clone, U: Clone>(arr1: &[T], arr2: &[U]) -> Vec<(T, U)> {
        arr1.iter()
            .zip(arr2.iter())
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }

    /// 数组解压（unzip）
    pub fn unzip<T: Clone, U: Clone>(arr: &[(T, U)]) -> (Vec<T>, Vec<U>) {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        
        for (a, b) in arr {
            vec1.push(a.clone());
            vec2.push(b.clone());
        }
        
        (vec1, vec2)
    }

    /// HashMap 工具：安全获取值
    pub fn map_get<K: Hash + Eq, V: Clone>(map: &HashMap<K, V>, key: &K) -> Option<V> {
        map.get(key).cloned()
    }

    /// HashMap 工具：获取值或默认值
    pub fn map_get_or_default<K: Hash + Eq, V: Clone + Default>(map: &HashMap<K, V>, key: &K) -> V {
        map.get(key).cloned().unwrap_or_default()
    }

    /// HashMap 工具：合并两个 HashMap
    pub fn map_merge<K: Hash + Eq + Clone, V: Clone>(
        map1: &HashMap<K, V>,
        map2: &HashMap<K, V>,
    ) -> HashMap<K, V> {
        let mut result = map1.clone();
        
        for (k, v) in map2 {
            result.insert(k.clone(), v.clone());
        }
        
        result
    }

    /// HashMap 工具：按值过滤
    pub fn map_filter_by_value<K: Hash + Eq + Clone, V: Clone, P>(
        map: &HashMap<K, V>,
        predicate: P,
    ) -> HashMap<K, V>
    where
        P: Fn(&V) -> bool,
    {
        map.iter()
            .filter(|(_, v)| predicate(v))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// HashMap 工具：按键过滤
    pub fn map_filter_by_key<K: Hash + Eq + Clone, V: Clone, P>(
        map: &HashMap<K, V>,
        predicate: P,
    ) -> HashMap<K, V>
    where
        P: Fn(&K) -> bool,
    {
        map.iter()
            .filter(|(k, _)| predicate(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// 计算频率
    pub fn frequency<T: Hash + Eq + Clone>(arr: &[T]) -> HashMap<T, usize> {
        let mut freq = HashMap::new();
        
        for item in arr {
            *freq.entry(item.clone()).or_insert(0) += 1;
        }
        
        freq
    }

    /// 找到最常见的元素
    pub fn most_frequent<T: Hash + Eq + Clone>(arr: &[T]) -> Option<T> {
        let freq = Self::frequency(arr);
        
        freq.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(item, _)| item)
    }

    /// 二分查找
    pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();
        
        while left < right {
            let mid = left + (right - left) / 2;
            
            match arr[mid].cmp(target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid,
            }
        }
        
        None
    }

    /// 快速排序
    pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        
        let pivot_index = Self::partition_for_sort(arr);
        let (left, right) = arr.split_at_mut(pivot_index);
        
        Self::quick_sort(left);
        Self::quick_sort(&mut right[1..]);
    }

    fn partition_for_sort<T: Ord>(arr: &mut [T]) -> usize {
        let pivot_index = arr.len() - 1;
        let mut i = 0;
        
        for j in 0..pivot_index {
            if arr[j] <= arr[pivot_index] {
                arr.swap(i, j);
                i += 1;
            }
        }
        
        arr.swap(i, pivot_index);
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        let unique = CollectionUtils::unique(&arr);
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn test_intersection() {
        let arr1 = vec![1, 2, 3, 4];
        let arr2 = vec![3, 4, 5, 6];
        let intersection = CollectionUtils::intersection(&arr1, &arr2);
        assert!(intersection.contains(&3));
        assert!(intersection.contains(&4));
        assert_eq!(intersection.len(), 2);
    }

    #[test]
    fn test_chunk() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7];
        let chunks = CollectionUtils::chunk(&arr, 3);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], vec![1, 2, 3]);
        assert_eq!(chunks[2], vec![7]);
    }

    #[test]
    fn test_frequency() {
        let arr = vec!['a', 'b', 'a', 'c', 'b', 'a'];
        let freq = CollectionUtils::frequency(&arr);
        assert_eq!(freq[&'a'], 3);
        assert_eq!(freq[&'b'], 2);
        assert_eq!(freq[&'c'], 1);
    }

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 5, 7, 9, 11];
        assert_eq!(CollectionUtils::binary_search(&arr, &5), Some(2));
        assert_eq!(CollectionUtils::binary_search(&arr, &4), None);
    }
}
