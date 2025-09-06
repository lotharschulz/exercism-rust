#[derive(Debug, Eq, Clone)]
pub struct CustomSet<T> {
    elements: Vec<T>,
}

impl<T> PartialEq for CustomSet<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }
        self.elements
            .iter()
            .all(|element| other.elements.contains(element))
    }
}

impl<T> CustomSet<T>
where
    T: PartialEq + Clone,
{
    pub fn new(input: &[T]) -> Self {
        let mut elements = Vec::new();
        for item in input {
            if !elements.contains(item) {
                elements.push(item.clone());
            }
        }
        CustomSet { elements }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.elements.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.elements.iter().all(|element| !other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let result_elements = self
            .elements
            .iter()
            .filter(|element| other.contains(element))
            .cloned()
            .collect();
        CustomSet {
            elements: result_elements,
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let result_elements = self
            .elements
            .iter()
            .filter(|element| !other.contains(element))
            .cloned()
            .collect();
        CustomSet {
            elements: result_elements,
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for element in &other.elements {
            result.add(element.clone());
        }
        result
    }
}
