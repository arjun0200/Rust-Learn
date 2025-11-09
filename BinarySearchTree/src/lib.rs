pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> BinarySearchTree<T> {
        //Create our root node
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value:T){
        if self.value.is_none(){
            self.value = Some(value)
        }
        else{
            match &self.value {
                None => (),
                Some(key) =>{
                    let target_node = 
                    if value < *key{
                        &mut self.left
                    }
                    else{
                        &mut self.right  //greater than
                    };
                    match target_node{
                        Some(ref mut node) => 
                            node.insert(value),
                
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }   
                    }    
                
            }
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    fn create() -> BinarySearchTree<i32>{
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(43);
        tree.insert(0);
        tree.insert(7);
        tree.insert(27);
        tree.insert(34);
        tree.insert(15);
        tree

    }
}