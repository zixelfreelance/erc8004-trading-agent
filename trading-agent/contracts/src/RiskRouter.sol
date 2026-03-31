// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol";
import "lib/openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";

interface IAgentIdentityRegistry {
    function getAgentWallet(uint256 agentId) external view returns (address);
    function ownerOf(uint256 tokenId) external view returns (address);
}

contract RiskRouter is EIP712 {
    IAgentIdentityRegistry public identityRegistry;

    bytes32 public constant TRADE_INTENT_TYPEHASH = keccak256(
        "TradeIntent(uint256 agentId,address agentWallet,string pair,uint8 action,uint128 amountUsdScaled,uint32 maxSlippageBps,uint64 nonce,uint64 deadline)"
    );

    // Nonce tracking per agent
    mapping(uint256 => mapping(uint64 => bool)) public usedNonces;

    // Risk limits (configurable by admin)
    address public admin;
    uint128 public maxPositionUsd = 1000 * 1e5;    // $1000 scaled by 1e5
    uint32 public maxSlippageBps = 100;              // 1%

    // Whitelisted pairs
    mapping(bytes32 => bool) public allowedPairs;

    event TradeApproved(
        uint256 indexed agentId,
        address indexed agentWallet,
        bytes32 intentHash,
        string pair,
        uint8 action,
        uint128 amountUsdScaled
    );

    event TradeRejected(
        uint256 indexed agentId,
        address indexed agentWallet,
        bytes32 intentHash,
        string reason
    );

    constructor(address _identityRegistry) EIP712("RiskRouter", "1") {
        identityRegistry = IAgentIdentityRegistry(_identityRegistry);
        admin = msg.sender;

        // Default whitelisted pairs
        allowedPairs[keccak256(bytes("BTCUSD"))] = true;
        allowedPairs[keccak256(bytes("ETHUSD"))] = true;
        allowedPairs[keccak256(bytes("SOLUSD"))] = true;
    }

    struct TradeIntent {
        uint256 agentId;
        address agentWallet;
        string pair;
        uint8 action;              // 0=buy, 1=sell
        uint128 amountUsdScaled;
        uint32 maxSlippageBps;
        uint64 nonce;
        uint64 deadline;
    }

    function submitIntent(
        TradeIntent calldata intent,
        bytes calldata signature
    ) external {
        // 1. Verify deadline
        if (block.timestamp > intent.deadline) {
            emit TradeRejected(intent.agentId, intent.agentWallet, bytes32(0), "expired");
            return;
        }

        // 2. Verify nonce
        if (usedNonces[intent.agentId][intent.nonce]) {
            emit TradeRejected(intent.agentId, intent.agentWallet, bytes32(0), "nonce reused");
            return;
        }

        // 3. Compute intent hash and verify signature
        bytes32 structHash = keccak256(abi.encode(
            TRADE_INTENT_TYPEHASH,
            intent.agentId,
            intent.agentWallet,
            keccak256(bytes(intent.pair)),
            intent.action,
            intent.amountUsdScaled,
            intent.maxSlippageBps,
            intent.nonce,
            intent.deadline
        ));
        bytes32 digest = _hashTypedDataV4(structHash);
        address signer = ECDSA.recover(digest, signature);

        // 4. Verify signer is the registered agent wallet
        address registeredWallet = identityRegistry.getAgentWallet(intent.agentId);
        if (signer != registeredWallet) {
            emit TradeRejected(intent.agentId, intent.agentWallet, digest, "invalid signer");
            return;
        }

        // 5. Check risk limits
        if (!allowedPairs[keccak256(bytes(intent.pair))]) {
            emit TradeRejected(intent.agentId, intent.agentWallet, digest, "pair not whitelisted");
            return;
        }

        if (intent.amountUsdScaled > maxPositionUsd) {
            emit TradeRejected(intent.agentId, intent.agentWallet, digest, "position too large");
            return;
        }

        if (intent.maxSlippageBps > maxSlippageBps) {
            emit TradeRejected(intent.agentId, intent.agentWallet, digest, "slippage too high");
            return;
        }

        // 6. Mark nonce used
        usedNonces[intent.agentId][intent.nonce] = true;

        // 7. Approve
        emit TradeApproved(
            intent.agentId,
            intent.agentWallet,
            digest,
            intent.pair,
            intent.action,
            intent.amountUsdScaled
        );
    }

    // Admin functions
    function setMaxPosition(uint128 _max) external {
        require(msg.sender == admin, "not admin");
        maxPositionUsd = _max;
    }

    function setMaxSlippage(uint32 _max) external {
        require(msg.sender == admin, "not admin");
        maxSlippageBps = _max;
    }

    function setAllowedPair(string calldata pair, bool allowed) external {
        require(msg.sender == admin, "not admin");
        allowedPairs[keccak256(bytes(pair))] = allowed;
    }
}
