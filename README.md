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

###### Prerequisites ：Ensure that you have Rust installed on your system.

The AST extraction is performed using the `rust_ast` module located at `src/process/rust_ast/src/main.rs`. To extract the AST, run the following command:

```shell
cargo run
```
#####  AST Simplification
The AST extracted from the Rust source code can be further simplified using the `simplifyast.py `script located at `safeNet/src/process/rust_asttree-json`. This script takes the AST and generates a simplified version. Please refer to the script for further details.

#####  Minimal AST Tree Construction and Traversal
The most simplified version of the AST is used to construct a minimal AST tree and perform traversal operations. This process is implemented in the `addnumtree.py` script located at `safeNet/src/process/rust_asttree-json`. The resulting tree can be used for input of machine learning.

Please refer to the respective source files for more information on how to use each component of this project.

- ### Data Flow Filter
##### Writing Filter Code:  
Write  or place the Rust code you wish to analyze and filter within the file `src/process/filter/tests`.  

##### Running Data Flow Analysis:  
Execute the `synparse_run` function present in `src/process/filter/parse_var.rs` to initiate data flow analysis.

##### Filtering Out Irrelevant Patterns:  
Review the results obtained after running `synparse_run` to identify and filter out irrelevant patterns as needed based on the data flow analysis performed.


- ### machine learning 

#####  Training and Testing Set Generation

The file `src/process/dataprocess/converttrainset.py` is used to read JSON files within a specified directory and randomly combine them to create training and testing sets. Each JSON file is paired with every other file, labeled as 1 if the patterns match and -1 if they don't. The dataset is split using a 3-fold method, generating corresponding training and testing CSV files.


##### Rust Code Similarity Detection using BERT and Siamese Neural Network (SNN)

This project focuses on training a BERT-based model on a large corpus of Rust source code for code similarity detection. The code similarity is determined by processing the data through a Siamese Neural Network (SNN).


##### BERT Preprocessing and Training

The Rust code is preprocessed and input to the BERT model for training. The code responsible for this process can be found in the `bert_pretrained.py` file located at `/safeNet/src/model/code/bert_pretrained.py`. This file handles the data preprocessing and training of the BERT model. Further details can be found within the script.

##### Siamese Neural Network (SNN)

The knowledge base data is preprocessed and transformed into pairs of either similar or dissimilar data for input to the SNN. The code responsible for this process can be found in the `bert_cos.py` file located at `/safeNet/src/model/code/bert_cos.py`. This file contains the implementation of the SNN and handles the input processing and training. Please refer to the script for more information on the SNN training process.

