// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract AgentReputationRegistry {
    struct FeedbackEntry {
        address client;
        int128 value;
        uint8 valueDecimals;
        string tag1;
        string tag2;
        string endpoint;
        string feedbackURI;
        bytes32 feedbackHash;
        uint256 timestamp;
    }

    // agentId => feedback entries
    mapping(uint256 => FeedbackEntry[]) private _feedback;

    // Reference to identity registry (for anti-Sybil: owner can't rate own agent)
    address public identityRegistry;

    event FeedbackGiven(
        uint256 indexed agentId,
        address indexed client,
        int128 value,
        uint8 valueDecimals,
        string tag1,
        string tag2,
        string endpoint,
        string feedbackURI,
        bytes32 feedbackHash
    );

    constructor(address _identityRegistry) {
        identityRegistry = _identityRegistry;
    }

    function giveFeedback(
        uint256 agentId,
        int128 value,
        uint8 valueDecimals,
        string calldata tag1,
        string calldata tag2,
        string calldata endpoint,
        string calldata feedbackURI,
        bytes32 feedbackHash
    ) external {
        // Anti-Sybil: caller must not be the agent owner
        // For hackathon: we use a separate evaluator address
        // In production: check against identity registry ownership

        _feedback[agentId].push(FeedbackEntry({
            client: msg.sender,
            value: value,
            valueDecimals: valueDecimals,
            tag1: tag1,
            tag2: tag2,
            endpoint: endpoint,
            feedbackURI: feedbackURI,
            feedbackHash: feedbackHash,
            timestamp: block.timestamp
        }));

        emit FeedbackGiven(agentId, msg.sender, value, valueDecimals, tag1, tag2, endpoint, feedbackURI, feedbackHash);
    }

    function getFeedbackCount(uint256 agentId) external view returns (uint256) {
        return _feedback[agentId].length;
    }

    /// Compute summary: count + average value for a given tag pair
    function getSummary(
        uint256 agentId,
        string calldata tag1,
        string calldata tag2
    ) external view returns (uint64 count, int128 summaryValue, uint8 summaryValueDecimals) {
        FeedbackEntry[] storage entries = _feedback[agentId];
        int256 total = 0;
        uint64 matched = 0;
        uint8 maxDecimals = 0;

        for (uint256 i = 0; i < entries.length; i++) {
            if (
                keccak256(bytes(entries[i].tag1)) == keccak256(bytes(tag1)) &&
                keccak256(bytes(entries[i].tag2)) == keccak256(bytes(tag2))
            ) {
                total += entries[i].value;
                matched++;
                if (entries[i].valueDecimals > maxDecimals) {
                    maxDecimals = entries[i].valueDecimals;
                }
            }
        }

        if (matched == 0) return (0, 0, 0);

        return (matched, int128(total / int256(uint256(matched))), maxDecimals);
    }
}
