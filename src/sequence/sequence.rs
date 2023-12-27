// Align the sequence to V & J genes

use pyo3::prelude::*;

#[pyclass(get_all, set_all)]
#[derive(Default, Clone, Debug)]
pub struct VJAlignment {
    // Structure containing the alignment between a V/J gene and the sequence
    // Note that the genes defined here include the palindromic insertion at the end

    // Example
    // gene (V):  ATACGATCATTGACAATCTGGAGATACGTA
    //                         ||||||\|||||\|\\\
    // sequence:               CAATCTAGAGATTCCAATCTAGAGATTCA
    // start_gene -------------^ 13
    // end_gene   ------------------------------^ 30
    // start_seq               0
    // end_seq                 -----------------^ 17
    // errors[u] contains the number of errors left if u nucleotides are removed
    // (starting from the end of the V-gene, or the beginning of the J gene)
    // the length of this vector is the maximum number of v/j deletion.
    // so here [5,4,3,2,2,1,1,1,1,1,1,0,0,...]
    pub index: usize, // index of the gene in the model
    pub start_seq: usize,
    pub end_seq: usize,
    pub start_gene: usize,
    pub end_gene: usize,
    pub errors: Vec<usize>,
}

#[pymethods]
impl VJAlignment {
    pub fn nb_errors(&self, del: usize) -> usize {
        if del >= self.errors.len() {
            return match self.errors.last() {
                None => 0,
                Some(l) => *l,
            };
        }
        self.errors[del]
    }
}

#[pyclass(get_all, set_all)]
#[derive(Default, Clone, Debug)]
pub struct DAlignment {
    // Structure containing the alignment between a D gene and the sequence
    // Similarly to VJaligment the gene include the palindromic insertion
    // d-gene  :         DDDDDDDDDDDD
    // sequence:     SSSSSSSSSSSSSSSSSSSSS
    // pos     ----------^ 4
    // error_left contains the number of errors in the alignment starting from the left
    // error_right contains the number of errors in the alignment starting from the right
    // For example:
    // DDDDDDDDDDD
    // SXSSSXSSXSX
    // errors_left:  [4,4,3,3,3,3,2,2,2,1,1]
    // errors_right: [4,3,3,2,2,2,1,1,1,1,0]
    // "pos" represents the start of the sequence *with palindromes added*
    pub index: usize,
    pub len_d: usize, // length of the D gene (with palindromic inserts)
    pub pos: usize,   // begining of the D-gene in the sequence (can't be < 0)
    pub errors_left: Vec<usize>,
    pub errors_right: Vec<usize>,
}

#[pymethods]
impl DAlignment {
    pub fn nb_errors(&self, deld3: usize, deld5: usize) -> usize {
        self.errors_left[deld5] + self.errors_right[deld3]
    }
    pub fn len(&self) -> usize {
        self.len_d
    }
    pub fn is_empty(&self) -> bool {
        self.len_d == 0
    }
}