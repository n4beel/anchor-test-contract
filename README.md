# Anchor Test Contract

## Overview

This is a comprehensive Solana smart contract built using the Anchor framework, specifically designed for testing static analysis tools. The contract focuses on Source Lines of Code (SLOC) counting and comment detection capabilities.

## Features

### Comment Types Included
- **Single-line comments** (`//`) - Standard line comments
- **Multi-line comments** (`/* */`) - Block comments spanning multiple lines
- **Documentation comments** (`///`) - Rust documentation comments
- **Inline comments** - Comments placed at the end of code lines
- **Header comments** - Large comment blocks for file/section headers

### Contract Functionality
- User account initialization with validation
- User profile updates (name, age)
- Token transfer operations between users
- Account deactivation for security
- Comprehensive error handling
- Event emission for off-chain monitoring

## Project Structure

```
anchor-test-contract/
├── Anchor.toml              # Anchor project configuration
├── Cargo.toml               # Workspace configuration
├── package.json             # Node.js dependencies
├── programs/
│   └── anchor-test-contract/
│       ├── Cargo.toml       # Program dependencies
│       └── src/
│           └── lib.rs       # Main contract code
└── README.md               # This file
```

## Building and Testing

### Prerequisites
- Rust 1.70+
- Solana CLI tools
- Anchor CLI
- Node.js and Yarn

### Build Commands
```bash
# Build the program
anchor build

# Run tests
anchor test

# Deploy to localnet
anchor deploy
```

## Static Analysis Testing

This contract is specifically designed to test various aspects of static analysis tools:

### SLOC Counting
- **Total lines**: ~350+ lines including comments
- **Code lines**: ~200+ lines of actual Rust code
- **Comment lines**: ~150+ lines of various comment types
- **Blank lines**: Strategic spacing for readability

### Comment Patterns
1. **File header**: Large multi-line comment block
2. **Function documentation**: Triple-slash comments (`///`)
3. **Inline explanations**: End-of-line comments
4. **Section dividers**: Large comment blocks separating code sections
5. **Code explanations**: Multi-line comments explaining complex logic

### Code Complexity
- Multiple instruction handlers
- Complex account validation contexts
- Error handling and custom error types
- Event definitions and emissions
- Utility functions and helpers

## Usage for Static Analysis

When testing your static analysis tool with this contract:

1. **SLOC Analysis**: Count different types of lines (code, comments, blank)
2. **Comment Detection**: Identify and classify different comment styles
3. **Complexity Metrics**: Analyze cyclomatic complexity and function sizes
4. **Security Patterns**: Detect common Solana/Anchor security patterns
5. **Documentation Coverage**: Measure documentation vs code ratio

## License

MIT License - See LICENSE file for details.

## Contributing

This is a test contract for static analysis tools. Contributions should focus on adding more diverse comment patterns and code structures for comprehensive testing.
# anchor-test-contract
