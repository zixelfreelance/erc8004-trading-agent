// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/AgentIdentityRegistry.sol";
import "../src/AgentReputationRegistry.sol";
import "../src/RiskRouter.sol";

contract DeployAll is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        AgentIdentityRegistry identity = new AgentIdentityRegistry();
        AgentReputationRegistry reputation = new AgentReputationRegistry(address(identity));
        RiskRouter router = new RiskRouter(address(identity));

        vm.stopBroadcast();

        console.log("Identity Registry:", address(identity));
        console.log("Reputation Registry:", address(reputation));
        console.log("Risk Router:", address(router));
    }
}
