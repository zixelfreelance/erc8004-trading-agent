// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/AgentIdentityRegistry.sol";

contract AgentIdentityRegistryTest is Test {
    AgentIdentityRegistry public registry;
    address public alice;
    address public bob;
    uint256 public walletKey;
    address public walletAddr;

    function setUp() public {
        registry = new AgentIdentityRegistry();
        alice = makeAddr("alice");
        bob = makeAddr("bob");
        (walletAddr, walletKey) = makeAddrAndKey("wallet");
    }

    function test_register_mints_nft() public {
        vm.prank(alice);
        uint256 agentId = registry.register();
        assertEq(agentId, 1);
        assertEq(registry.ownerOf(agentId), alice);
    }

    function test_register_with_uri() public {
        vm.prank(alice);
        uint256 agentId = registry.registerWithURI("ipfs://QmTest123");
        assertEq(registry.agentURI(agentId), "ipfs://QmTest123");
    }

    function test_update_uri() public {
        vm.prank(alice);
        uint256 agentId = registry.registerWithURI("ipfs://old");

        vm.prank(alice);
        registry.updateAgentURI(agentId, "ipfs://new");
        assertEq(registry.agentURI(agentId), "ipfs://new");
    }

    function test_update_uri_not_owner_reverts() public {
        vm.prank(alice);
        uint256 agentId = registry.register();

        vm.prank(bob);
        vm.expectRevert("Not agent owner");
        registry.updateAgentURI(agentId, "ipfs://hacked");
    }

    function test_set_agent_wallet() public {
        vm.prank(alice);
        uint256 agentId = registry.register();

        uint256 deadline = block.timestamp + 1 hours;

        bytes32 structHash = keccak256(abi.encode(
            registry.WALLET_AUTH_TYPEHASH(),
            agentId,
            walletAddr,
            deadline
        ));

        bytes32 domainSeparator = _computeDomainSeparator();
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(walletKey, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.prank(alice);
        registry.setAgentWallet(agentId, walletAddr, deadline, signature);
        assertEq(registry.getAgentWallet(agentId), walletAddr);
    }

    function test_wallet_cleared_on_transfer() public {
        vm.prank(alice);
        uint256 agentId = registry.register();

        // Set wallet first
        uint256 deadline = block.timestamp + 1 hours;
        bytes32 structHash = keccak256(abi.encode(
            registry.WALLET_AUTH_TYPEHASH(),
            agentId,
            walletAddr,
            deadline
        ));
        bytes32 domainSeparator = _computeDomainSeparator();
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(walletKey, digest);

        vm.prank(alice);
        registry.setAgentWallet(agentId, walletAddr, deadline, abi.encodePacked(r, s, v));
        assertEq(registry.getAgentWallet(agentId), walletAddr);

        // Transfer NFT from alice to bob
        vm.prank(alice);
        registry.transferFrom(alice, bob, agentId);

        // Wallet should be cleared
        assertEq(registry.getAgentWallet(agentId), address(0));
        assertEq(registry.ownerOf(agentId), bob);
    }

    function test_sequential_ids() public {
        vm.prank(alice);
        uint256 id1 = registry.register();

        vm.prank(bob);
        uint256 id2 = registry.register();

        vm.prank(alice);
        uint256 id3 = registry.registerWithURI("ipfs://three");

        assertEq(id1, 1);
        assertEq(id2, 2);
        assertEq(id3, 3);
    }

    function _computeDomainSeparator() internal view returns (bytes32) {
        return keccak256(abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            keccak256("AgentIdentityRegistry"),
            keccak256("1"),
            block.chainid,
            address(registry)
        ));
    }
}
