use super::*;
use std::collections::HashSet;

// An Output represents an output in a transaction
#[derive(Clone)]
pub struct Output {
    // The address to which the output is being sent
    pub to_addr: Address,
    // The value of the output
    pub value: u64,
}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.to_addr == other.to_addr &&
        self.value == other.value
    }
}

impl Hashable for Output {
    // Returns the bytes of the Output, which is used to compute its hash
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

         // Add the bytes of the address to the vector
        bytes.extend(self.to_addr.as_bytes());
         // Add the bytes of the value to the vector
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

// A Transaction represents a transaction in the blockchain
pub struct Transaction {
    // The inputs to the transaction
    pub inputs: Vec<Output>,
    // The outputs of the transaction
    pub outputs: Vec<Output>,
}

impl Clone for Transaction {
    fn clone(&self) -> Self {
        Transaction {
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
        }
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.inputs == other.inputs &&
        self.outputs == other.outputs
    }
}

impl Transaction {
    // Returns the sum of the values of the inputs
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }
    // Returns the sum of the values of the outputs
    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }
    // Returns a set of the hashes of the inputs
    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }
    // Returns a set of the hashes of the outputs
    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }
    // Returns true if the transaction is a coinbase transaction (has no inputs)
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    // Returns the bytes of the transaction, which are used to compute its hash
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        // Add the bytes of the inputs to the vector
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );
        // Add the bytes of the outputs to the vector
        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes
    }
}
