"""
Kraken CLI Agent Skills Mapping
Maps ERC-8004 trading strategies to Kraken CLI's 50+ pre-built agent skills
"""

from typing import Dict, List, Optional
from dataclasses import dataclass
import json

@dataclass
class KrakenSkill:
    """Represents a Kraken CLI agent skill"""
    name: str
    description: str
    category: str
    risk_level: str  # low, medium, high
    required_services: List[str]
    erc8004_mapping: Optional[str] = None

class KrakenSkillsMapper:
    """Maps ERC-8004 strategies to Kraken CLI skills"""
    
    def __init__(self):
        self.skills = self._initialize_skills()
        self.strategy_mappings = self._create_strategy_mappings()
    
    def _initialize_skills(self) -> Dict[str, KrakenSkill]:
        """Initialize Kraken CLI skills based on available commands"""
        skills = {
            # Market Analysis Skills
            "market_overview": KrakenSkill(
                name="market_overview",
                description="Get comprehensive market overview with key metrics",
                category="analysis",
                risk_level="low",
                required_services=["market"],
                erc8004_mapping="market_analysis_strategy"
            ),
            "price_alerts": KrakenSkill(
                name="price_alerts",
                description="Set up price alerts for multiple assets",
                category="monitoring",
                risk_level="low",
                required_services=["market"],
                erc8004_mapping="price_monitoring_strategy"
            ),
            "volume_analysis": KrakenSkill(
                name="volume_analysis",
                description="Analyze trading volume patterns and trends",
                category="analysis",
                risk_level="low",
                required_services=["market"],
                erc8004_mapping="volume_based_strategy"
            ),
            "technical_indicators": KrakenSkill(
                name="technical_indicators",
                description="Calculate technical indicators from OHLCV data",
                category="analysis",
                risk_level="low",
                required_services=["market"],
                erc8004_mapping="technical_analysis_strategy"
            ),
            
            # Portfolio Management Skills
            "portfolio_rebalance": KrakenSkill(
                name="portfolio_rebalance",
                description="Rebalance portfolio based on target allocations",
                category="portfolio",
                risk_level="medium",
                required_services=["account", "trade"],
                erc8004_mapping="portfolio_rebalance_strategy"
            ),
            "risk_assessment": KrakenSkill(
                name="risk_assessment",
                description="Assess current portfolio risk exposure",
                category="risk",
                risk_level="low",
                required_services=["account"],
                erc8004_mapping="risk_management_strategy"
            ),
            "position_sizing": KrakenSkill(
                name="position_sizing",
                description="Calculate optimal position sizes based on risk",
                category="risk",
                risk_level="medium",
                required_services=["account"],
                erc8004_mapping="position_sizing_strategy"
            ),
            "performance_tracking": KrakenSkill(
                name="performance_tracking",
                description="Track portfolio performance over time",
                category="portfolio",
                risk_level="low",
                required_services=["account"],
                erc8004_mapping="performance_tracking_strategy"
            ),
            
            # Trading Strategy Skills
            "mean_reversion": KrakenSkill(
                name="mean_reversion",
                description="Mean reversion trading strategy",
                category="trading",
                risk_level="medium",
                required_services=["market", "trade", "paper"],
                erc8004_mapping="mean_reversion_strategy"
            ),
            "momentum_trading": KrakenSkill(
                name="momentum_trading",
                description="Momentum-based trading strategy",
                category="trading",
                risk_level="medium",
                required_services=["market", "trade", "paper"],
                erc8004_mapping="momentum_strategy"
            ),
            "arbitrage_detection": KrakenSkill(
                name="arbitrage_detection",
                description="Detect arbitrage opportunities across markets",
                category="trading",
                risk_level="high",
                required_services=["market"],
                erc8004_mapping="arbitrage_strategy"
            ),
            "breakout_trading": KrakenSkill(
                name="breakout_trading",
                description="Trade breakout patterns from consolidation",
                category="trading",
                risk_level="medium",
                required_services=["market", "trade", "paper"],
                erc8004_mapping="breakout_strategy"
            ),
            
            # Risk Management Skills
            "stop_loss_manager": KrakenSkill(
                name="stop_loss_manager",
                description="Manage stop-loss orders for open positions",
                category="risk",
                risk_level="medium",
                required_services=["account", "trade"],
                erc8004_mapping="stop_loss_strategy"
            ),
            "take_profit_manager": KrakenSkill(
                name="take_profit_manager",
                description="Manage take-profit targets",
                category="risk",
                risk_level="medium",
                required_services=["account", "trade"],
                erc8004_mapping="take_profit_strategy"
            ),
            "drawdown_monitor": KrakenSkill(
                name="drawdown_monitor",
                description="Monitor portfolio drawdown and trigger alerts",
                category="risk",
                risk_level="low",
                required_services=["account"],
                erc8004_mapping="drawdown_monitoring_strategy"
            ),
            "correlation_analysis": KrakenSkill(
                name="correlation_analysis",
                description="Analyze correlation between assets",
                category="risk",
                risk_level="low",
                required_services=["market"],
                erc8004_mapping="correlation_strategy"
            ),
            
            # Advanced Trading Skills
            "options_greeks": KrakenSkill(
                name="options_greeks",
                description="Calculate options Greeks for derivatives trading",
                category="derivatives",
                risk_level="high",
                required_services=["market", "trade"],
                erc8004_mapping="options_strategy"
            ),
            "futures_spread": KrakenSkill(
                name="futures_spread",
                description="Futures spread trading strategies",
                category="derivatives",
                risk_level="high",
                required_services=["market", "trade"],
                erc8004_mapping="futures_spread_strategy"
            ),
            "leverage_optimization": KrakenSkill(
                name="leverage_optimization",
                description="Optimize leverage usage for maximum returns",
                category="derivatives",
                risk_level="high",
                required_services=["account", "trade"],
                erc8004_mapping="leverage_strategy"
            ),
            
            # Monitoring Skills
            "real_time_dashboard": KrakenSkill(
                name="real_time_dashboard",
                description="Real-time market and portfolio dashboard",
                category="monitoring",
                risk_level="low",
                required_services=["market", "account"],
                erc8004_mapping="dashboard_strategy"
            ),
            "alert_system": KrakenSkill(
                name="alert_system",
                description="Custom alert system for market events",
                category="monitoring",
                risk_level="low",
                required_services=["market"],
                erc8004_mapping="alert_system_strategy"
            ),
            "news_sentiment": KrakenSkill(
                name="news_sentiment",
                description="Analyze news sentiment for market impact",
                category="analysis",
                risk_level="low",
                required_services=["market"],
                erc8004_mapping="sentiment_analysis_strategy"
            ),
            
            # Execution Skills
            "twap_execution": KrakenSkill(
                name="twap_execution",
                description="Time-weighted average price execution",
                category="execution",
                risk_level="medium",
                required_services=["trade"],
                erc8004_mapping="twap_strategy"
            ),
            "vwap_execution": KrakenSkill(
                name="vwap_execution",
                description="Volume-weighted average price execution",
                category="execution",
                risk_level="medium",
                required_services=["market", "trade"],
                erc8004_mapping="vwap_strategy"
            ),
            "iceberg_orders": KrakenSkill(
                name="iceberg_orders",
                description="Execute large orders via iceberg strategy",
                category="execution",
                risk_level="medium",
                required_services=["trade"],
                erc8004_mapping="iceberg_strategy"
            )
        }
        return skills
    
    def _create_strategy_mappings(self) -> Dict[str, List[str]]:
        """Create mappings from ERC-8004 strategies to Kraken skills"""
        return {
            # ERC-8004 Strategy -> Compatible Kraken Skills
            "conservative_strategy": [
                "market_overview",
                "risk_assessment",
                "performance_tracking",
                "real_time_dashboard"
            ],
            "balanced_strategy": [
                "market_overview",
                "portfolio_rebalance",
                "risk_assessment",
                "position_sizing",
                "mean_reversion",
                "real_time_dashboard"
            ],
            "aggressive_strategy": [
                "momentum_trading",
                "breakout_trading",
                "leverage_optimization",
                "arbitrage_detection",
                "real_time_dashboard"
            ],
            "arbitrage_strategy": [
                "arbitrage_detection",
                "correlation_analysis",
                "vwap_execution",
                "real_time_dashboard"
            ],
            "market_making_strategy": [
                "volume_analysis",
                "technical_indicators",
                "iceberg_orders",
                "real_time_dashboard"
            ],
            "trend_following_strategy": [
                "momentum_trading",
                "breakout_trading",
                "technical_indicators",
                "stop_loss_manager",
                "real_time_dashboard"
            ],
            "mean_reversion_strategy": [
                "mean_reversion",
                "technical_indicators",
                "risk_assessment",
                "position_sizing",
                "real_time_dashboard"
            ]
        }
    
    def get_skills_for_strategy(self, erc8004_strategy: str) -> List[KrakenSkill]:
        """Get Kraken skills compatible with an ERC-8004 strategy"""
        skill_names = self.strategy_mappings.get(erc8004_strategy, [])
        return [self.skills[name] for name in skill_names if name in self.skills]
    
    def get_skill_by_name(self, skill_name: str) -> Optional[KrakenSkill]:
        """Get a specific skill by name"""
        return self.skills.get(skill_name)
    
    def get_skills_by_risk_level(self, risk_level: str) -> List[KrakenSkill]:
        """Get all skills with a specific risk level"""
        return [skill for skill in self.skills.values() if skill.risk_level == risk_level]
    
    def get_skills_by_category(self, category: str) -> List[KrakenSkill]:
        """Get all skills in a specific category"""
        return [skill for skill in self.skills.values() if skill.category == category]
    
    def validate_strategy_skill_compatibility(self, 
                                           erc8004_strategy: str, 
                                           agent_reputation: int,
                                           available_services: List[str]) -> Dict:
        """
        Validate if an ERC-8004 strategy is compatible with available skills
        based on agent reputation and available Kraken services
        """
        compatible_skills = []
        incompatible_skills = []
        
        for skill in self.get_skills_for_strategy(erc8004_strategy):
            # Check if agent has sufficient reputation for skill risk level
            reputation_thresholds = {"low": 0, "medium": 50, "high": 80}
            required_reputation = reputation_thresholds.get(skill.risk_level, 0)
            
            # Check if required services are available
            has_services = all(service in available_services for service in skill.required_services)
            
            if agent_reputation >= required_reputation and has_services:
                compatible_skills.append(skill)
            else:
                incompatible_skills.append({
                    "skill": skill,
                    "reason": "Insufficient reputation" if agent_reputation < required_reputation else "Missing services"
                })
        
        return {
            "strategy": erc8004_strategy,
            "agent_reputation": agent_reputation,
            "compatible_skills": compatible_skills,
            "incompatible_skills": incompatible_skills,
            "compatibility_score": len(compatible_skills) / (len(compatible_skills) + len(incompatible_skills))
        }
    
    def get_skill_recommendations(self, 
                                agent_profile: Dict,
                                market_conditions: Dict) -> List[KrakenSkill]:
        """
        Get skill recommendations based on agent profile and market conditions
        """
        recommendations = []
        
        # Base recommendations on agent experience
        experience = agent_profile.get("experience", "beginner")
        risk_tolerance = agent_profile.get("risk_tolerance", "low")
        
        # Market condition adaptations
        volatility = market_conditions.get("volatility", "normal")
        trend = market_conditions.get("trend", "sideways")
        
        # Filter skills by risk tolerance
        suitable_risk_levels = {
            "low": ["low"],
            "medium": ["low", "medium"],
            "high": ["low", "medium", "high"]
        }
        
        for skill in self.skills.values():
            if skill.risk_level in suitable_risk_levels.get(risk_tolerance, ["low"]):
                # Add market condition specific skills
                if volatility == "high" and skill.category in ["risk", "monitoring"]:
                    recommendations.append(skill)
                elif trend == "bullish" and skill.name in ["momentum_trading", "breakout_trading"]:
                    recommendations.append(skill)
                elif trend == "bearish" and skill.name in ["mean_reversion", "risk_assessment"]:
                    recommendations.append(skill)
                elif trend == "sideways" and skill.name in ["arbitrage_detection", "market_overview"]:
                    recommendations.append(skill)
        
        return recommendations
    
    def export_skill_mappings(self) -> Dict:
        """Export skill mappings for documentation or external use"""
        return {
            "skills": {name: {
                "name": skill.name,
                "description": skill.description,
                "category": skill.category,
                "risk_level": skill.risk_level,
                "required_services": skill.required_services,
                "erc8004_mapping": skill.erc8004_mapping
            } for name, skill in self.skills.items()},
            "strategy_mappings": self.strategy_mappings
        }

# Example usage
def example_skill_mapping():
    """Example of using the skills mapper"""
    mapper = KrakenSkillsMapper()
    
    # Get skills for a balanced strategy
    balanced_skills = mapper.get_skills_for_strategy("balanced_strategy")
    print(f"Skills for balanced strategy: {len(balanced_skills)}")
    
    # Validate strategy compatibility
    validation = mapper.validate_strategy_skill_compatibility(
        erc8004_strategy="aggressive_strategy",
        agent_reputation=85,
        available_services=["market", "account", "trade", "paper"]
    )
    
    print(f"Compatibility score: {validation['compatibility_score']:.2%}")
    print(f"Compatible skills: {len(validation['compatible_skills'])}")
    
    # Get recommendations
    agent_profile = {"experience": "intermediate", "risk_tolerance": "medium"}
    market_conditions = {"volatility": "normal", "trend": "bullish"}
    
    recommendations = mapper.get_skill_recommendations(agent_profile, market_conditions)
    print(f"Recommended skills: {len(recommendations)}")
    
    # Export mappings
    mappings = mapper.export_skill_mappings()
    print(f"Total skills mapped: {len(mappings['skills'])}")

if __name__ == "__main__":
    example_skill_mapping()
