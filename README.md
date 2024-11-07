# FederNear: Privacy-Preserving Federated Learning Platform

FederNear is a decentralized platform built on the NEAR blockchain, enabling secure, privacy-preserving model training on distributed datasets. It ensures that sensitive data remains private by leveraging Pedersen commitments to keep model parameters (weights and biases) confidential during training.



NEAR blockchain enables decentralized coordination, allowing nodes to participate in the training process while supporting on-chain aggregation of encrypted data.


## Key Features

### Blockchain Coordination (NEAR)
The NEAR blockchain is integral to orchestrating the training process. It emits events, manages node registration, and facilitates the aggregation of encrypted data on-chain.

### Privacy-Preserving Model Training
Pedersen commitments allow nodes to train models on their local datasets while ensuring confidentiality of both data and model parameters. This approach protects sensitive information during the entire training process.

### Decentralized Node Participation
Users can register as nodes to contribute to model training, leveraging their private datasets without revealing any data to others. This decentralized system encourages collaborative learning without the need to share data.

### Encrypted Aggregation
Once nodes train their models and submit encrypted weights and biases, NEAR-based smart contracts aggregate these encrypted values on-chain, maintaining privacy throughout the aggregation process.

## Architecture Overview

### Server-Side (Model Provider)

#### Upload Model:
The user (model provider) uploads the model (e.g., linear regression, neural network) and registers it on the NEAR blockchain.

#### Event Emission:
Once the model is registered, an event is emitted on the NEAR blockchain, notifying all registered nodes of the new model available for training.

### Client-Side (Nodes)

#### Node Registration:
Users can register their wallet address as nodes on the NEAR blockchain.

#### Data Processing:
When a node detects the model registration event, it retrieves the model parameters, loads its private dataset, and initiates training.

#### Pedersen Commitments:
The trained model’s weights and biases are encrypted using Pedersen commitments, ensuring confidentiality.

#### Encrypted Submission:
Nodes submit their encrypted weights and biases to the NEAR smart contract for aggregation.

### On-Chain Aggregation
The NEAR smart contract aggregates encrypted results from multiple nodes without exposing any underlying data.

### Decrypted Results
The model provider retrieves the final encrypted aggregated model and decrypts the weights using their private key.

Through federated learning, FederNear enables secure, decentralized, and privacy-preserving model training, ensuring that sensitive information remains private and secure across the network of participating nodes.
