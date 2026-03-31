// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/AgentIdentityRegistry.sol";
import "../src/RiskRouter.sol";

contract RiskRouterTest is Test {
    AgentIdentityRegistry public identity;
    RiskRouter public router;

    address public owner;
    uint256 public agentWalletKey;
    address public agentWalletAddr;
    uint256 public agentId;

    function setUp() public {
        owner = makeAddr("owner");
        (agentWalletAddr, agentWalletKey) = makeAddrAndKey("agentWallet");

        identity = new AgentIdentityRegistry();
        router = new RiskRouter(address(identity));

        // Register agent and set wallet
        vm.prank(owner);
        agentId = identity.register();

        uint256 deadline = block.timestamp + 1 hours;
        bytes32 structHash = keccak256(abi.encode(
            identity.WALLET_AUTH_TYPEHASH(),
            agentId,
            agentWalletAddr,
            deadline
        ));
        bytes32 domainSeparator = _computeIdentityDomainSeparator();
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(agentWalletKey, digest);

        vm.prank(owner);
        identity.setAgentWallet(agentId, agentWalletAddr, deadline, abi.encodePacked(r, s, v));
    }

    function _signIntent(RiskRouter.TradeIntent memory intent, uint256 privKey) internal view returns (bytes memory) {
        bytes32 structHash = keccak256(abi.encode(
            router.TRADE_INTENT_TYPEHASH(),
            intent.agentId,
            intent.agentWallet,
            keccak256(bytes(intent.pair)),
            intent.action,
            intent.amountUsdScaled,
            intent.maxSlippageBps,
            intent.nonce,
            intent.deadline
        ));
        bytes32 domainSeparator = _computeRouterDomainSeparator();
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privKey, digest);
        return abi.encodePacked(r, s, v);
    }

    function _defaultIntent() internal view returns (RiskRouter.TradeIntent memory) {
        return RiskRouter.TradeIntent({
            agentId: agentId,
            agentWallet: agentWalletAddr,
            pair: "BTCUSD",
            action: 0,
            amountUsdScaled: 500 * 1e5,
            maxSlippageBps: 50,
            nonce: 1,
            deadline: uint64(block.timestamp + 1 hours)
        });
    }

    function test_submit_valid_intent() public {
        RiskRouter.TradeIntent memory intent = _defaultIntent();
        bytes memory sig = _signIntent(intent, agentWalletKey);

        vm.expectEmit(true, true, false, false);
        emit RiskRouter.TradeApproved(agentId, agentWalletAddr, bytes32(0), "BTCUSD", 0, 500 * 1e5);

        router.submitIntent(intent, sig);

        assertTrue(router.usedNonces(agentId, 1));
    }

    function test_expired_intent_rejected() public {
        RiskRouter.TradeIntent memory intent = _defaultIntent();
        intent.deadline = uint64(block.timestamp - 1);
        bytes memory sig = _signIntent(intent, agentWalletKey);

        vm.expectEmit(true, true, false, false);
        emit RiskRouter.TradeRejected(agentId, agentWalletAddr, bytes32(0), "expired");

        router.submitIntent(intent, sig);
    }

    function test_reused_nonce_rejected() public {
        RiskRouter.TradeIntent memory intent = _defaultIntent();
        bytes memory sig = _signIntent(intent, agentWalletKey);

        router.submitIntent(intent, sig);

        // Second submission with same nonce
        vm.expectEmit(true, true, false, false);
        emit RiskRouter.TradeRejected(agentId, agentWalletAddr, bytes32(0), "nonce reused");

        router.submitIntent(intent, sig);
    }

    function test_invalid_signer_rejected() public {
        (, uint256 wrongKey) = makeAddrAndKey("wrongWallet");

        RiskRouter.TradeIntent memory intent = _defaultIntent();
        bytes memory sig = _signIntent(intent, wrongKey);

        vm.expectEmit(true, true, false, false);
        emit RiskRouter.TradeRejected(agentId, agentWalletAddr, bytes32(0), "invalid signer");

        router.submitIntent(intent, sig);
    }

    function test_position_too_large_rejected() public {
        RiskRouter.TradeIntent memory intent = _defaultIntent();
        intent.amountUsdScaled = 2000 * 1e5; // exceeds 1000 * 1e5 limit
        bytes memory sig = _signIntent(intent, agentWalletKey);

        vm.expectEmit(true, true, false, false);
        emit RiskRouter.TradeRejected(agentId, agentWalletAddr, bytes32(0), "position too large");

        router.submitIntent(intent, sig);
    }

    function test_pair_not_whitelisted_rejected() public {
        RiskRouter.TradeIntent memory intent = _defaultIntent();
        intent.pair = "DOGEUSD";
        bytes memory sig = _signIntent(intent, agentWalletKey);

        vm.expectEmit(true, true, false, false);
        emit RiskRouter.TradeRejected(agentId, agentWalletAddr, bytes32(0), "pair not whitelisted");

        router.submitIntent(intent, sig);
    }

    function _computeIdentityDomainSeparator() internal view returns (bytes32) {
        return keccak256(abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            keccak256("AgentIdentityRegistry"),
            keccak256("1"),
            block.chainid,
            address(identity)
        ));
    }

    function _computeRouterDomainSeparator() internal view returns (bytes32) {
        return keccak256(abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            keccak256("RiskRouter"),
            keccak256("1"),
            block.chainid,
            address(router)
        ));
    }
}
