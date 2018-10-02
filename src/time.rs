
pub type Time = u64;

type Timed<T> = (Time, T);

pub struct Timeline<T> {
    timeline: Vec<Timed<T>>,
    default: T,
}

impl<T> Timeline<T> {
    pub fn new(default: T) -> Timeline<T> {
        Timeline {
            timeline: Vec::new(),
            default,
        }
    }
    
    /// Inserts `value` at the point `time` on the timeline.
    pub fn insert(&mut self, time: Time, value: T) {
        let insert_index = self.lookup_index(&time).map_or(0, |index| index + 1);
        
        self.timeline.insert(insert_index, (time, value));
    }
    
    /// Returns the value at the point on the timeline with the greatest time less than `time`.
    /// Returns `default` if `time` is smaller than the smallest time.
    pub fn lookup(&self, time: &Time) -> &T {
        self.lookup_index(time).map_or(&self.default, |index| &self.timeline[index].1)
    }
    
    /// Returns the index of the point on the timeline with the greatest time less than `time`.
    /// Returns `None` if `time` is smaller than the smallest time.
    fn lookup_index(&self, time: &Time) -> Option<usize> {
        if self.timeline.len() == 0 {
            return None;
        }
        
        let mut low = 0usize;
        let mut high = self.timeline.len() - 1;
    
        while low <= high {
            let mut mid = (low + high) / 2;
        
            if mid >= self.timeline.len() - 1 {
                return Some(self.timeline.len() - 1);
            }
        
            let mid_time = &self.timeline[mid].0;
            let after_mid_time = &self.timeline[mid + 1].0;
        
            if mid_time <= time && time <= after_mid_time {
                if time == after_mid_time {
                    return Some(mid + 1);
                }
                
                return Some(mid);
            } else if time < mid_time {
                if mid == 0 {
                    return None;
                }
                
                high = mid - 1;
            } else if after_mid_time < time {
                low = mid + 1;
            }
        }
    
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn lookup_index() {
        let timeline = Timeline {
            timeline: vec![(2, 0.3), (4, 0.2), (6, 0.5)],
            default: 0.1,
        };
    
        assert_eq!(0, timeline.lookup_index(&2).unwrap());
        assert_eq!(0, timeline.lookup_index(&3).unwrap());
        assert_eq!(1, timeline.lookup_index(&4).unwrap());
        assert_eq!(1, timeline.lookup_index(&5).unwrap());
        assert_eq!(2, timeline.lookup_index(&6).unwrap());
        assert_eq!(2, timeline.lookup_index(&7).unwrap());
        
        assert!(timeline.lookup_index(&1).is_none());
    }
    
    #[test]
    fn lookup() {
        let timeline = Timeline {
            timeline: vec![(2, 0.3), (4, 0.2), (6, 0.5)],
            default: 0.1,
        };
        
        assert_eq!(&0.3, timeline.lookup(&2));
        assert_eq!(&0.3, timeline.lookup(&3));
        assert_eq!(&0.2, timeline.lookup(&4));
        assert_eq!(&0.2, timeline.lookup(&5));
        assert_eq!(&0.5, timeline.lookup(&6));
        assert_eq!(&0.5, timeline.lookup(&7));
        
        assert_eq!(&0.1, timeline.lookup(&1));
    }
    
    #[test]
    fn insert() {
        let mut timeline = Timeline::new(0.1);
        
        timeline.insert(4, 0.2);
        timeline.insert(6, 0.5);
        timeline.insert(2, 0.3);
        timeline.insert(3, 0.25);
        
        assert_eq!(vec![(2, 0.3), (3, 0.25), (4, 0.2), (6, 0.5)], timeline.timeline);
    }
}
