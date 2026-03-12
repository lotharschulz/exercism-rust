use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct ComputeCell<'a, T> {
    value: T,
    dependencies: Vec<CellId>,
    compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
}

pub struct Reactor<'a, T> {
    input_cells: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    callbacks: HashMap<ComputeCellId, HashMap<CallbackId, Box<dyn FnMut(T) + 'a>>>,
    next_input_id: usize,
    next_compute_id: usize,
    next_callback_id: usize,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + 'a> Reactor<'a, T> {
    /// Creates a new empty reactor.
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
            callbacks: HashMap::new(),
            next_input_id: 0,
            next_compute_id: 0,
            next_callback_id: 0,
        }
    }

    /// Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(self.next_input_id);
        self.next_input_id += 1;
        self.input_cells.insert(id, initial);
        id
    }

    /// Creates a compute cell with the specified dependencies and compute function.
    /// The compute function is expected to take in its arguments in the same order as specified in
    /// `dependencies`.
    /// You do not need to reject compute functions that expect more arguments than there are
    /// dependencies (how would you check for this, anyway?).
    ///
    /// If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    /// (If multiple dependencies do not exist, exactly which one is returned is not defined and
    /// will not be tested)
    ///
    /// Notice that there is no way to *remove* a cell.
    /// This means that you may assume, without checking, that if the dependencies exist at creation
    /// time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        // Verify all dependencies exist
        for &dep in dependencies {
            if !self.cell_exists(dep) {
                return Err(dep);
            }
        }

        // Compute the initial value
        let values = self.get_dependency_values(dependencies);
        let value = compute_func(&values);

        let id = ComputeCellId(self.next_compute_id);
        self.next_compute_id += 1;

        self.compute_cells.insert(
            id,
            ComputeCell {
                value,
                dependencies: dependencies.to_vec(),
                compute_func: Box::new(compute_func),
            },
        );

        Ok(id)
    }

    /// Retrieves the current value of the cell, or None if the cell does not exist.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(input_id) => self.input_cells.get(&input_id).copied(),
            CellId::Compute(compute_id) => self.compute_cells.get(&compute_id).map(|cell| cell.value),
        }
    }

    /// Sets the value of the specified input cell.
    /// Returns false if the cell does not exist.
    /// When an input value is changed, all dependent compute cells are updated,
    /// and their callbacks are triggered if the values change.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if !self.input_cells.contains_key(&id) {
            return false;
        }

        self.input_cells.insert(id, new_value);
        
        // Save old values of all compute cells before propagation
        let old_values: HashMap<ComputeCellId, T> = self
            .compute_cells
            .iter()
            .map(|(&id, cell)| (id, cell.value))
            .collect();

        // Propagate changes
        self.propagate_changes();

        // Call callbacks for cells that changed
        for (&id, &old_value) in &old_values {
            let new_value = self.compute_cells[&id].value;
            if new_value != old_value {
                if let Some(callbacks) = self.callbacks.get_mut(&id) {
                    let callback_ids: Vec<_> = callbacks.keys().copied().collect();
                    for cb_id in callback_ids {
                        if let Some(callback) = callbacks.get_mut(&cb_id) {
                            callback(new_value);
                        }
                    }
                }
            }
        }

        true
    }

    /// Adds a callback to the specified compute cell.
    /// Returns the ID of the just-added callback, or None if the cell doesn't exist.
    ///
    /// Callbacks on input cells will not be tested.
    ///
    /// The semantics of callbacks (as will be tested):
    /// For a single set_value call, each compute cell's callbacks should each be called:
    /// * Zero times if the compute cell's value did not change as a result of the set_value call.
    /// * Exactly once if the compute cell's value changed as a result of the set_value call.
    ///   The value passed to the callback should be the final value of the compute cell after the
    ///   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if !self.compute_cells.contains_key(&id) {
            return None;
        }

        let callback_id = CallbackId(self.next_callback_id);
        self.next_callback_id += 1;

        self.callbacks
            .entry(id)
            .or_insert_with(HashMap::new)
            .insert(callback_id, Box::new(callback));

        Some(callback_id)
    }

    /// Removes the specified callback, using an ID returned from add_callback.
    /// Returns an Err if either the cell or callback does not exist.
    /// A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if !self.compute_cells.contains_key(&cell) {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        self.callbacks
            .get_mut(&cell)
            .and_then(|callbacks| callbacks.remove(&callback).map(|_| ()))
            .ok_or(RemoveCallbackError::NonexistentCallback)
    }

    /// Checks if a cell exists in the reactor.
    fn cell_exists(&self, id: CellId) -> bool {
        match id {
            CellId::Input(input_id) => self.input_cells.contains_key(&input_id),
            CellId::Compute(compute_id) => self.compute_cells.contains_key(&compute_id),
        }
    }

    /// Gets the current values of a list of dependency cells.
    fn get_dependency_values(&self, dependencies: &[CellId]) -> Vec<T> {
        dependencies
            .iter()
            .map(|&dep| self.value(dep).unwrap())
            .collect()
    }

    /// Propagates changes through the dependency graph.
    /// Updates all compute cells based on their dependencies.
    /// This is done in multiple passes to ensure all transitive dependencies are properly updated.
    fn propagate_changes(&mut self) {
        // Keep updating until no more changes occur (handles transitive dependencies)
        loop {
            let compute_ids: Vec<_> = self.compute_cells.keys().copied().collect();
            let mut any_changed = false;

            for &id in &compute_ids {
                if self.update_compute_cell(id) {
                    any_changed = true;
                }
            }

            if !any_changed {
                break;
            }
        }
    }

    /// Updates a single compute cell with its new value based on dependencies.
    /// Returns true if the value changed, false otherwise.
    fn update_compute_cell(&mut self, id: ComputeCellId) -> bool {
        let old_value = self.compute_cells[&id].value;

        // Save the dependencies and compute function before we need to borrow mutably
        let (dependencies, new_value) = {
            let cell = &self.compute_cells[&id];
            let dependencies = cell.dependencies.clone();
            let values = self.get_dependency_values(&dependencies);
            let new_value = (cell.compute_func)(&values);
            (dependencies, new_value)
        };

        // Check if value actually changed
        if new_value == old_value {
            return false;
        }

        // Update the value
        self.compute_cells.get_mut(&id).unwrap().value = new_value;
        true
    }
}
