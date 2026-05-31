# lau-adinkra

Akan geometric behavioral signatures. The Adinkra symbols of the Akan people encode philosophical principles in geometric form — Sankofa ("go back and fetch it"), Gye Nyame ("except God"), and dozens more. Each symbol is a behavioral instruction rendered as geometry.

## The concept in 60 seconds

Adinkra symbols aren't decoration — they're compressed behavioral protocols. Each symbol encodes a proverb, a teaching, and a set of geometric elements. This crate implements them as typed data:

- **Symbol meanings** — the proverb, teaching, and element count for each Adinkra
- **Geometric structure** — the shapes, symmetries, and topology of each symbol
- **Behavioral signatures** — each symbol maps to an agent behavior pattern
- **Symbol composition** — combine symbols to create compound behavioral protocols
- **Classification** — group symbols by theme (wisdom, courage, community, etc.)

## Quick start

```rust
use lau_adinkra::{AdinkraSymbol, AdinkraRegistry, SymbolMeaning};

// Access the full Adinkra registry
let registry = AdinkraRegistry::new();

// Look up Sankofa — "go back and fetch it"
let sankofa = registry.get("sankofa").unwrap();
assert_eq!(sankofa.meaning.proverb, "Go back and fetch it");

// Geometric properties
let geo = &sankofa.geometry;
println!("Elements: {}, Symmetry: {:?}", geo.elements, geo.symmetry);

// Get all symbols in a category
let wisdom = registry.by_theme("wisdom");
let community = registry.by_theme("community");

// Compose two symbols into a compound behavioral protocol
let compound = sankofa.compose(registry.get("gye_nyame").unwrap());
```

## Key types

| Type | What it is |
|------|-----------|
| `AdinkraSymbol` | A complete Adinkra with meaning, geometry, and behavior |
| `AdinkraRegistry` | The full catalog of known Adinkra symbols |
| `SymbolMeaning` | The proverb, teaching, and element count |
| `SymbolGeometry` | Shapes, symmetries, and topological properties |
| `BehavioralSignature` | The agent behavior pattern encoded by a symbol |

## Contributing

[Open an issue](https://github.com/SuperInstance/lau-adinkra/issues) or PR.
