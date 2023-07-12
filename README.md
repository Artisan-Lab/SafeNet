## Info
 
This project aims at mitigating the usage of Unsafe Rust Code though code review and refactor or replacement. We want to recommend useful safe code samples to developers based on their current Rust functions with unsafe code.

## Project Structure
### knowledgebase
A minimal code base with useful unsafe-safe (if exist) Rust code pairs.

### testdata
Unsafe code samples collected from GitHub

### src
Source code of code pre-processing and machine learning components.

- ### pre-processing
####  Rust AST Extraction and JSON Generation

This project aims to extract the Abstract Syntax Tree (AST) from Rust source code, process it, simplify it, build a tree structure, and generate a corresponding JSON file.

#####  AST Extraction

The AST extraction is performed using the `rust_ast` module located at `safeNet/src/process/rust_ast`. To extract the AST, run the following command:

```shell
cargo run
```
#####  AST Simplification
The AST extracted from the Rust source code can be further simplified using the `astsimpletree.py `script located at `safeNet/src/process/rust_asttree-json`. This script takes the AST and generates a simplified version. Please refer to the script for further details.

#####  Minimal AST Tree Construction and Traversal
The most simplified version of the AST is used to construct a minimal AST tree and perform traversal operations. This process is implemented in the `addnumtree.py` script located at `safeNet/src/process/rust_asttree-json`. The resulting tree can be used for various purposes.

Please refer to the respective source files for more information on how to use each component of this project.


- ### machine learning 


##### Rust Code Similarity Detection using BERT and Siamese Neural Network (SNN)

This project focuses on training a BERT-based model on a large corpus of Rust source code for code similarity detection. The code similarity is determined by processing the data through a Siamese Neural Network (SNN).


##### BERT Preprocessing and Training

The Rust code is preprocessed and input to the BERT model for training. The code responsible for this process can be found in the `bert_pretrained.py` file located at `/safeNet/src/model/code/bert_pretrained.py`. This file handles the data preprocessing and training of the BERT model. Further details can be found within the script.

##### Siamese Neural Network (SNN)

The knowledge base data is preprocessed and transformed into pairs of either similar or dissimilar data for input to the SNN. The code responsible for this process can be found in the `bert_cos.py` file located at `/safeNet/src/model/code/bert_cos.py`. This file contains the implementation of the SNN and handles the input processing and training. Please refer to the script for more information on the SNN training process.

