use std::error::Error;
#[derive(Debug)]
pub struct Dataset {
    pub data: Vec<Vec<f64>>,
    pub target: Vec<f64>,
}

impl Dataset {
    pub fn new(data: Vec<Vec<f64>>, target: Vec<f64>) -> Self {
        Dataset { data, target }
    }
    pub fn split_at_point(&self, mid: usize) -> (Self, Self) {
        if mid > self.target.len() {
            panic!("Mid point is greater than lenght!");
        }
        let (data1, data2) = self.data.iter().map(|x| x.split_at(mid)).fold(
            (Vec::new(), Vec::new()),
            |(mut acc1, mut acc2), (x1, x2)| {
                acc1.push(x1.to_vec());
                acc2.push(x2.to_vec());
                (acc1, acc2)
            },
        );
        let (target1, target2) = self.target.split_at(mid);
        (
            Dataset::new(data1, target1.to_vec()),
            Dataset::new(data2, target2.to_vec()),
        )
    }
    pub fn len(&self) -> usize {
        self.target.len()
    }

    pub fn dim(&self) -> usize {
        self.data[0].len()
    }
    pub fn get(&self, index: usize) -> (&Vec<f64>, f64) {
        (&self.data[index], self.target[index])
    }
    pub fn get_data(&self, index: usize) -> &Vec<f64> {
        &self.data[index]
    }
    pub fn get_target(&self, index: usize) -> f64 {
        self.target[index]
    }
    pub fn num_features(&self) -> usize {
        self.data.len()
    }
}

impl TryFrom<(Vec<Vec<f64>>, Vec<f64>)> for Dataset {
    type Error = Box<dyn Error>;
    fn try_from(value: (Vec<Vec<f64>>, Vec<f64>)) -> Result<Self, Self::Error> {
        Ok(Dataset::new(value.0, value.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dataset() -> Result<Dataset, Box<dyn Error>> {
        let syntetic_data = (
            vec![vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 5.0]],
            vec![3.0, 2.0, 1.0],
        );
        let dataset = Dataset::try_from(syntetic_data);
        return dataset;
    }
    #[test]
    fn check_if_ds_is_ok() {
        let dataset = create_dataset();
        assert!(dataset.is_ok());
    }
    #[test]
    fn check_split() {
        let dataset = create_dataset();
        let (new, dataset) = dataset.unwrap().split_at_point(1);
        assert_eq!(vec![vec![1.0], vec![2.0]], new.data);
        assert_eq!(vec![3.0], new.target);
        assert_eq!(vec![vec![2.0, 3.0], vec![4.0, 5.0]], dataset.data);
        assert_eq!(vec![2.0, 1.0], dataset.target);
    }
    #[test]
    fn general_testings() {
        let dataset = create_dataset().unwrap();
        assert_eq!(dataset.len(), 3);
        assert_eq!(dataset.dim(), 3);
        assert_eq!(dataset.get(0), (&vec![1.0, 2.0, 3.0], 3.0));
        assert_eq!(dataset.get_data(0), &vec![1.0, 2.0, 3.0]);
        assert_eq!(dataset.get_target(0), 3.0);
        assert_eq!(dataset.num_features(), 2);
        let (split1, split2) = dataset.split_at_point(1);
        assert_eq!(split1.len(), 1);
        assert_eq!(split2.len(), 2);
        assert_eq!(split1.dim(), 1);
        assert_eq!(split2.dim(), 2);
        assert_eq!(split1.get(0), (&vec![1.0], 3.0));
        assert_eq!(split2.get(0), (&vec![2.0, 3.0], 2.0));
    }
}
