// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import "lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol";
import "lib/openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";

contract AgentIdentityRegistry is ERC721, EIP712 {
    uint256 private _nextAgentId = 1;

    // Agent URI (points to Agent Card JSON on IPFS/HTTPS)
    mapping(uint256 => string) private _agentURIs;

    // Agent hot wallet (set via EIP-712 signed authorization)
    mapping(uint256 => address) private _agentWallets;

    // EIP-712 typehash for wallet authorization
    bytes32 public constant WALLET_AUTH_TYPEHASH = keccak256(
        "AgentWalletAuthorization(uint256 agentId,address newWallet,uint256 deadline)"
    );

    event AgentRegistered(uint256 indexed agentId, address indexed owner, string agentURI);
    event AgentURIUpdated(uint256 indexed agentId, string newAgentURI);
    event AgentWalletSet(uint256 indexed agentId, address indexed wallet);
    event AgentWalletUnset(uint256 indexed agentId);

    constructor() ERC721("AgentIdentity", "AGENT") EIP712("AgentIdentityRegistry", "1") {}

    function registerWithURI(string calldata agentURI) external returns (uint256 agentId) {
        agentId = _nextAgentId++;
        _mint(msg.sender, agentId);
        _agentURIs[agentId] = agentURI;
        emit AgentRegistered(agentId, msg.sender, agentURI);
    }

    function register() external returns (uint256 agentId) {
        agentId = _nextAgentId++;
        _mint(msg.sender, agentId);
        emit AgentRegistered(agentId, msg.sender, "");
    }

    function updateAgentURI(uint256 agentId, string calldata newAgentURI) external {
        require(ownerOf(agentId) == msg.sender, "Not agent owner");
        _agentURIs[agentId] = newAgentURI;
        emit AgentURIUpdated(agentId, newAgentURI);
    }

    function agentURI(uint256 agentId) external view returns (string memory) {
        ownerOf(agentId); // reverts if nonexistent
        return _agentURIs[agentId];
    }

    function setAgentWallet(
        uint256 agentId,
        address newWallet,
        uint256 deadline,
        bytes calldata signature
    ) external {
        require(ownerOf(agentId) == msg.sender, "Not agent owner");
        require(block.timestamp <= deadline, "Authorization expired");

        bytes32 structHash = keccak256(abi.encode(
            WALLET_AUTH_TYPEHASH,
            agentId,
            newWallet,
            deadline
        ));
        bytes32 digest = _hashTypedDataV4(structHash);
        address signer = ECDSA.recover(digest, signature);
        require(signer == newWallet, "Invalid wallet signature");

        _agentWallets[agentId] = newWallet;
        emit AgentWalletSet(agentId, newWallet);
    }

    function unsetAgentWallet(uint256 agentId) external {
        require(ownerOf(agentId) == msg.sender, "Not agent owner");
        delete _agentWallets[agentId];
        emit AgentWalletUnset(agentId);
    }

    function getAgentWallet(uint256 agentId) external view returns (address) {
        return _agentWallets[agentId];
    }

    function _update(address to, uint256 tokenId, address auth) internal override returns (address) {
        address from = super._update(to, tokenId, auth);
        if (from != address(0) && from != to) {
            delete _agentWallets[tokenId];
            emit AgentWalletUnset(tokenId);
        }
        return from;
    }
}
