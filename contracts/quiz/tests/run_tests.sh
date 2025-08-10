#!/bin/bash

# Test runner script
set -e

echo "🧪 Running Quiz Room Contract Tests"
echo "=================================="

# Run basic unit tests
echo "📋 Running unit tests..."
cargo test --lib tests::test_contract_initialization -- --nocapture
cargo test --lib tests::test_token_management -- --nocapture
cargo test --lib tests::test_emergency_controls -- --nocapture

# Run room creation tests
echo "🏠 Running room creation tests..."
cargo test --lib tests::test_pool_room_creation -- --nocapture
cargo test --lib tests::test_pool_room_creation_validation -- --nocapture
cargo test --lib tests::test_asset_room_creation -- --nocapture

# Run player management tests
echo "👥 Running player management tests..."
cargo test --lib tests::test_player_joining -- --nocapture
cargo test --lib tests::test_player_joining_validation -- --nocapture

# Run game completion tests
echo "🏆 Running game completion tests..."
cargo test --lib tests::test_room_completion_with_winners -- --nocapture
cargo test --lib tests::test_room_completion_by_screen_names -- --nocapture
cargo test --lib tests::test_room_completion_validation -- --nocapture

# Run financial tests
echo "💰 Running financial calculation tests..."
cargo test --lib tests::test_financial_calculations -- --nocapture

# Run security tests
echo "🔒 Running security tests..."
cargo test --lib tests::test_integer_overflow_protection -- --nocapture
cargo test --lib tests::test_input_validation -- --nocapture

# Run performance tests
echo "⚡ Running performance tests..."
cargo test --lib tests::test_large_room_performance -- --nocapture

# Run edge case tests
echo "🎯 Running edge case tests..."
cargo test --lib tests::test_edge_cases -- --nocapture

# Run snapshot tests
echo "📸 Running snapshot tests..."
cargo test --lib snapshot_tests -- --nocapture

# Run property-based tests
echo "🔬 Running property-based tests..."
cargo test --lib property_tests -- --nocapture

# Run integration tests
echo "🔗 Running integration tests..."
cargo test --lib integration_tests::test_complete_quiz_game_flow -- --nocapture
cargo test --lib integration_tests::test_multi_room_concurrent_operations -- --nocapture

# Run error scenario tests
echo "❌ Running error scenario tests..."
cargo test --lib error_scenario_tests -- --nocapture

echo ""
echo "✅ All tests completed successfully!"
echo ""
echo "📊 Test Summary:"
echo "  - Unit tests: Core functionality"
echo "  - Integration tests: Complete workflows"
echo "  - Security tests: Overflow protection, validation"
echo "  - Performance tests: Gas optimization verification"
echo "  - Snapshot tests: State verification"
echo "  - Error tests: Edge cases and failures"