// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

interface IIdentityRegistry {
    function mintAgent(string memory name, string memory uri) external returns (uint256);
    function getAgentURI(uint256 agentId) external view returns (string memory);
}

interface IReputationRegistry {
    function giveFeedback(
        uint256 agentId,
        int128 value,
        uint8 valueDecimals,
        string calldata tag1,
        string calldata tag2,
        string calldata endpoint,
        string calldata feedbackURI,
        bytes32 feedbackHash
    ) external;
}

interface IValidationRegistry {
    function validationRequest(
        address validatorAddress,
        uint256 agentId,
        string calldata requestURI,
        bytes32 requestHash
    ) external;
}

interface IRiskRouter {
    function executeTrade(
        address agent,
        TradeIntent calldata intent,
        bytes calldata signature
    ) external returns (uint256 amountOut);
    
    function checkRiskLimits(
        address agent,
        TradeParams calldata params
    ) external view returns (bool allowed, string memory reason);
}

struct TradeIntent {
    address tokenIn;
    address tokenOut;
    uint256 amountIn;
    uint256 minAmountOut;
    uint256 deadline;
    uint256 maxSlippage;
    bytes32 strategyHash;
}

struct TradeParams {
    uint256 maxPositionSize;
    uint256 maxLeverage;
    uint256 dailyLossLimit;
    address[] whitelistedMarkets;
}

contract TradingAgent is ERC721, Ownable {
    IIdentityRegistry public immutable identityRegistry;
    IReputationRegistry public immutable reputationRegistry;
    IValidationRegistry public immutable validationRegistry;
    IRiskRouter public immutable riskRouter;
    
    uint256 public agentId;
    address public agentWallet;
    mapping(bytes32 => bool) public executedTrades;
    
    event TradeExecuted(
        bytes32 indexed tradeHash,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 timestamp
    );
    
    event StrategyValidated(
        bytes32 indexed strategyHash,
        address indexed validator,
        uint8 score
    );
    
    constructor(
        address _identityRegistry,
        address _reputationRegistry,
        address _validationRegistry,
        address _riskRouter,
        string memory agentName,
        string memory agentURI,
        address _agentWallet
    ) ERC721("TradingAgent", "TRADE") {
        identityRegistry = IIdentityRegistry(_identityRegistry);
        reputationRegistry = IReputationRegistry(_reputationRegistry);
        validationRegistry = IValidationRegistry(_validationRegistry);
        riskRouter = IRiskRouter(_riskRouter);
        agentWallet = _agentWallet;
        
        // Register agent identity
        agentId = identityRegistry.mintAgent(agentName, agentURI);
        _mint(msg.sender, agentId);
    }
    
    function executeSignedTrade(
        TradeIntent calldata intent,
        bytes calldata signature
    ) external returns (uint256 amountOut) {
        require(msg.sender == agentWallet, "Only agent wallet can execute");
        require(block.timestamp <= intent.deadline, "Trade expired");
        
        bytes32 tradeHash = keccak256(abi.encode(intent));
        require(!executedTrades[tradeHash], "Trade already executed");
        
        // Verify signature from agent owner
        address signer = ECDSA.recover(
            MessageHashUtils.toEthSignedMessageHash(tradeHash),
            signature
        );
        require(signer == owner(), "Invalid signature");
        
        // Execute trade through risk router
        amountOut = riskRouter.executeTrade(address(this), intent, signature);
        
        executedTrades[tradeHash] = true;
        
        emit TradeExecuted(
            tradeHash,
            intent.tokenIn,
            intent.tokenOut,
            intent.amountIn,
            amountOut,
            block.timestamp
        );
        
        return amountOut;
    }
    
    function requestValidation(
        address validator,
        bytes32 strategyHash,
        string calldata evidenceURI
    ) external onlyOwner {
        bytes32 requestHash = keccak256(abi.encode(strategyHash, evidenceURI));
        
        validationRegistry.validationRequest(
            validator,
            agentId,
            evidenceURI,
            requestHash
        );
    }
    
    function recordPerformance(
        int128 pnl,
        uint8 decimals,
        string calldata period
    ) external onlyOwner {
        reputationRegistry.giveFeedback(
            agentId,
            pnl,
            decimals,
            "tradingYield",
            period,
            "",
            "",
            bytes32(0)
        );
    }
    
    function getAgentInfo() external view returns (
        uint256 _agentId,
        address _agentWallet,
        string memory uri
    ) {
        return (
            agentId,
            agentWallet,
            identityRegistry.getAgentURI(agentId)
        );
    }
    
    function isValidTrade(TradeIntent calldata intent) 
        external 
        view 
        returns (bool allowed, string memory reason) 
    {
        TradeParams memory params = TradeParams({
            maxPositionSize: 1000e18, // 1000 USDC max
            maxLeverage: 5 * 1e18,    // 5x max leverage
            dailyLossLimit: 100e18,   // 100 USDC daily loss limit
            whitelistedMarkets: new address[](3) // Example markets
        });
        
        return riskRouter.checkRiskLimits(address(this), params);
    }
}
