# lau-adinkra

> Adinkra symbols as compressed behavioral signatures for PLATO agents — West African visual philosophy meets agent personality encoding.

## What This Does

This crate maps agent behavioral profiles (patience, precision, playfulness, conservation affinity, social tendency) to Adinkra symbols — the visual philosophical language of the Akan people of Ghana. Each agent's 5D behavioral vector gets projected into a geometric symbol with symmetry, complexity, and a proverb. The registry tracks which agents map to which symbols and can cluster agents by behavioral similarity.

## The Key Idea

An Adinkra symbol is compressed information. In Akan culture, each symbol encodes a proverb, a teaching, and a worldview. This crate uses that same principle: an agent's behavioral profile is a point in 5D space. The `to_adinkra()` method projects that point onto a symbol — patient + precise agents get radial symmetry, playful + social agents get spirals, conservation-focused agents get concentric rings. The symbol isn't decoration; it's a lossy compression of the agent's character that a human can read at a glance.

## Install

```bash
cargo add lau-adinkra
```

## Quick Start

```rust
use lau_adinkra::*;

// Define an agent's behavioral profile
let sig = AgentSignature::new("alpha", 0.8, 0.9, 0.3, 0.5, 0.4);

// Convert to an Adinkra symbol
let symbol = sig.to_adinkra();
println!("{}: {} — {}", symbol.name, symbol.meaning.proverb, symbol.meaning.teaching);
// Patient + precise → Radial symmetry with 8 arms, reflection symmetry
assert!(symbol.is_balanced());

// Register in the central registry
let mut registry = AdinkraRegistry::default();
registry.register_agent(sig.clone());
let stats = registry.stats();
println!("{} agents, {} symbols, avg complexity {:.2}", 
    stats.total_agents, stats.total_symbols, stats.avg_complexity);
```

## API Reference

### `AdinkraId`
Unique symbol identifier. Wraps a `String`. Methods: `new(s)`, `as_str()`.

### `SymbolMeaning`
Philosophical content. Fields: `name`, `proverb`, `teaching`, `elements` (structural complexity count).

### `SymbolGeometry`
Geetric structure. Variants: `Spiral`, `Radial { arms }`, `Interlocking { count }`, `Concentric { rings }`, `Asymmetric`.

### `AdinkraSymbol`
A complete symbol. Fields: `id`, `name`, `meaning`, `symmetry_fold`, `has_reflection`, `complexity`, `geometry`.
- `is_balanced() -> bool` — True if ≥2-fold rotational symmetry or has reflection.
- `encode_complexity() -> f64` — Derived from element count and geometry type.

### `AgentSignature`
5D behavioral profile. Fields: `agent_id`, `patience`, `precision`, `playfulness`, `conservation_affinity`, `social_tendency` (all clamped to [0,1]).
- `new(agent_id, patience, precision, playfulness, conservation, social) -> Self`
- `distance_to(&self, other: &AgentSignature) -> f64` — Euclidean distance in 5D.
- `is_similar(&self, other, threshold) -> bool` — Distance within threshold.
- `to_adinkra(&self) -> AdinkraSymbol` — Project behavioral profile onto a symbol.

### `AdinkraRegistry`
Central registry. Fields: `symbols: HashMap<AdinkraId, AdinkraSymbol>`, `signatures: HashMap<String, AgentSignature>`.
- `register_agent(&mut self, sig: AgentSignature)` — Register and auto-generate symbol.
- `get_symbol(&self, agent_id) -> Option<&AdinkraSymbol>`
- `find_similar(&self, sig: &AgentSignature, threshold: f64) -> Vec<&AgentSignature>`
- `stats(&self) -> AdinkraStats` — Compute registry statistics.

### `AdinkraStats`
Registry stats. Fields: `total_symbols`, `total_agents`, `avg_complexity`, `balanced_ratio`, `cluster_count`.

## How It Works

**Behavior → Geometry mapping:**
- High patience + precision → `Radial` (ordered, focused) with arms = precision × 8
- High playfulness + social → `Spiral` (dynamic, creative)
- High conservation → `Concentric` rings (layered, deep)
- High playfulness + low social → `Interlocking` (complex individual)
- Default → `Asymmetric`

**Complexity encoding:** Behavioral variance across the 5 axes. A flat profile (all traits equal) is low complexity; a spiky profile (one dominant trait) is high complexity.

**Proverb selection:** Based on the dominant trait — patience gets river proverbs, precision gets arrow proverbs, playfulness gets spider proverbs, etc.

## The Math

The 5D behavioral space ℝ⁵ is projected onto a discrete symbol space via:
- **Complexity:** σ(behavioral vector) = √(Σᵢ(xᵢ - μ)² / 5) / 0.5, normalized to [0,1]
- **Geometry:** Decision tree partition of the unit hypercube [0,1]⁵
- **Distance:** Standard Euclidean distance ‖x - y‖₂ for similarity clustering

## Testing

**42 tests** covering: AdinkraId, all geometry variants, symbol balance/complexity, AgentSignature creation/clamping, distance computation, similarity, to_adinkra projection for all behavioral profiles, registry operations, stats computation, clustering, serde round-trips.

## License

MIT
