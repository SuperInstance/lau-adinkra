use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// 1. AdinkraId
// ---------------------------------------------------------------------------

/// Unique identifier for an Adinkra symbol.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdinkraId(pub String);

impl AdinkraId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// 2. SymbolMeaning
// ---------------------------------------------------------------------------

/// The philosophical meaning behind a symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMeaning {
    pub name: String,
    pub proverb: String,
    pub teaching: String,
    pub elements: u32,
}

// ---------------------------------------------------------------------------
// 3. SymbolGeometry + AdinkraSymbol
// ---------------------------------------------------------------------------

/// Geometric structure of an Adinkra symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolGeometry {
    Spiral,
    Radial { arms: u32 },
    Interlocking { count: u32 },
    Concentric { rings: u32 },
    Asymmetric,
}

/// A single Adinkra symbol encoding a philosophical concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdinkraSymbol {
    pub id: AdinkraId,
    pub name: String,
    pub meaning: SymbolMeaning,
    pub symmetry_fold: u32,
    pub has_reflection: bool,
    pub complexity: f64,
    pub geometry: SymbolGeometry,
}

impl AdinkraSymbol {
    /// A symbol is balanced if it has at least 2-fold rotational symmetry
    /// or reflection symmetry.
    pub fn is_balanced(&self) -> bool {
        self.symmetry_fold >= 2 || self.has_reflection
    }

    /// Derive complexity from element count and geometry.
    pub fn encode_complexity(&self) -> f64 {
        let element_factor = (self.meaning.elements as f64 / 20.0).min(1.0);
        let geometry_factor = match &self.geometry {
            SymbolGeometry::Spiral => 0.7,
            SymbolGeometry::Radial { arms } => (*arms as f64 / 12.0).min(1.0),
            SymbolGeometry::Interlocking { count } => (*count as f64 / 8.0).min(1.0),
            SymbolGeometry::Concentric { rings } => (*rings as f64 / 6.0).min(1.0),
            SymbolGeometry::Asymmetric => 0.5,
        };
        (element_factor * 0.6 + geometry_factor * 0.4).min(1.0)
    }
}

// ---------------------------------------------------------------------------
// 4. AgentSignature
// ---------------------------------------------------------------------------

/// Behavioral profile of an agent, convertible into an Adinkra symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSignature {
    pub agent_id: String,
    pub patience: f64,
    pub precision: f64,
    pub playfulness: f64,
    pub conservation_affinity: f64,
    pub social_tendency: f64,
}

impl AgentSignature {
    fn clamp01(v: f64) -> f64 {
        v.clamp(0.0, 1.0)
    }

    /// Create a new signature, clamping all values to [0, 1].
    pub fn new(
        agent_id: impl Into<String>,
        patience: f64,
        precision: f64,
        playfulness: f64,
        conservation_affinity: f64,
        social_tendency: f64,
    ) -> Self {
        Self {
            agent_id: agent_id.into(),
            patience: Self::clamp01(patience),
            precision: Self::clamp01(precision),
            playfulness: Self::clamp01(playfulness),
            conservation_affinity: Self::clamp01(conservation_affinity),
            social_tendency: Self::clamp01(social_tendency),
        }
    }

    /// Euclidean distance between two behavioral signatures.
    pub fn distance_to(&self, other: &AgentSignature) -> f64 {
        let d2 = (self.patience - other.patience).powi(2)
            + (self.precision - other.precision).powi(2)
            + (self.playfulness - other.playfulness).powi(2)
            + (self.conservation_affinity - other.conservation_affinity).powi(2)
            + (self.social_tendency - other.social_tendency).powi(2);
        d2.sqrt()
    }

    /// Check if two signatures are similar within a threshold.
    pub fn is_similar(&self, other: &AgentSignature, threshold: f64) -> bool {
        self.distance_to(other) <= threshold
    }

    /// Convert behavioral profile into a geometric Adinkra symbol.
    ///
    /// The key insight: the symbol is not decoration — it's compressed
    /// information about the agent's character.
    pub fn to_adinkra(&self) -> AdinkraSymbol {
        let complexity = self.encode_behavioral_complexity();
        let (geometry, symmetry_fold, has_reflection) = self.derive_geometry();

        let name = format!("{}-adinkra", self.agent_id);
        let proverb = self.derive_proverb();

        AdinkraSymbol {
            id: AdinkraId::new(&self.agent_id),
            name: name.clone(),
            meaning: SymbolMeaning {
                name,
                proverb,
                teaching: format!(
                    "Agent {} — patience:{:.2} precision:{:.2} play:{:.2} conservation:{:.2} social:{:.2}",
                    self.agent_id,
                    self.patience,
                    self.precision,
                    self.playfulness,
                    self.conservation_affinity,
                    self.social_tendency
                ),
                elements: (complexity * 20.0).ceil() as u32,
            },
            symmetry_fold,
            has_reflection,
            complexity,
            geometry,
        }
    }

    fn encode_behavioral_complexity(&self) -> f64 {
        // More spread-out profiles are more complex.
        let mean = (self.patience
            + self.precision
            + self.playfulness
            + self.conservation_affinity
            + self.social_tendency)
            / 5.0;
        let variance = ((self.patience - mean).powi(2)
            + (self.precision - mean).powi(2)
            + (self.playfulness - mean).powi(2)
            + (self.conservation_affinity - mean).powi(2)
            + (self.social_tendency - mean).powi(2))
            / 5.0;
        // sqrt(variance) max ≈ 0.5 when all mass on one axis
        (variance.sqrt() / 0.5).min(1.0)
    }

    fn derive_geometry(&self) -> (SymbolGeometry, u32, bool) {
        // Patient + precise agents get radial symmetry (order, focus)
        if self.patience > 0.6 && self.precision > 0.6 {
            let arms = (self.precision * 8.0).ceil() as u32;
            return (SymbolGeometry::Radial { arms }, arms, true);
        }
        // Playful + social agents get spiral (dynamic, creative)
        if self.playfulness > 0.5 && self.social_tendency > 0.5 {
            return (SymbolGeometry::Spiral, 1, false);
        }
        // Conservation-heavy agents get concentric rings (layered, deep)
        if self.conservation_affinity > 0.6 {
            let rings = (self.conservation_affinity * 4.0).ceil() as u32;
            return (SymbolGeometry::Concentric { rings }, rings, true);
        }
        // High playfulness + low social = interlocking (complex individual)
        if self.playfulness > 0.4 && self.social_tendency < 0.3 {
            let count = (self.playfulness * 6.0).ceil() as u32;
            return (SymbolGeometry::Interlocking { count }, count, false);
        }
        // Default: asymmetric
        (SymbolGeometry::Asymmetric, 1, false)
    }

    fn derive_proverb(&self) -> String {
        if self.patience > 0.7 {
            "The river does not drink its own water".to_string()
        } else if self.precision > 0.7 {
            "The sharpened arrow finds its mark".to_string()
        } else if self.playfulness > 0.7 {
            "The spider's web is never complete".to_string()
        } else if self.conservation_affinity > 0.7 {
            "The baobab holds what the wind forgets".to_string()
        } else if self.social_tendency > 0.7 {
            "One tree does not make a forest".to_string()
        } else {
            "The pattern speaks for itself".to_string()
        }
    }
}

// ---------------------------------------------------------------------------
// 5 & 6. AdinkraRegistry + AdinkraStats
// ---------------------------------------------------------------------------

/// Statistics about the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdinkraStats {
    pub total_symbols: usize,
    pub total_agents: usize,
    pub avg_complexity: f64,
    pub balanced_ratio: f64,
    pub cluster_count: usize,
}

/// Central registry mapping agents to their Adinkra symbols.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdinkraRegistry {
    pub symbols: HashMap<AdinkraId, AdinkraSymbol>,
    pub signatures: HashMap<String, AgentSignature>,
}

impl AdinkraRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_symbol(&mut self, symbol: AdinkraSymbol) {
        self.symbols.insert(symbol.id.clone(), symbol);
    }

    pub fn register_agent(&mut self, agent_id: &str, signature: AgentSignature) {
        let adinkra = signature.to_adinkra();
        self.symbols.insert(adinkra.id.clone(), adinkra);
        self.signatures.insert(agent_id.to_string(), signature);
    }

    /// Get the Adinkra symbol for a registered agent.
    pub fn agent_symbol(&self, agent_id: &str) -> Option<&AdinkraSymbol> {
        self.symbols.get(&AdinkraId::new(agent_id))
    }

    /// Find agents whose signatures are within `threshold` distance of the given agent.
    pub fn find_similar_agents(&self, agent_id: &str, threshold: f64) -> Vec<&str> {
        if let Some(sig) = self.signatures.get(agent_id) {
            self.signatures
                .keys()
                .filter(|&id| id != agent_id)
                .filter(|id| {
                    self.signatures.get(id.as_str())
                        .map(|other| sig.is_similar(other, threshold))
                        .unwrap_or(false)
                })
                .map(|s| s.as_str())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Measure how diverse the symbol set is (0 = all identical, 1 = maximally diverse).
    pub fn symbol_diversity(&self) -> f64 {
        let sigs: Vec<&AgentSignature> = self.signatures.values().collect();
        if sigs.len() < 2 {
            return 0.0;
        }
        let mut total = 0.0;
        let mut count = 0u32;
        for i in 0..sigs.len() {
            for j in (i + 1)..sigs.len() {
                total += sigs[i].distance_to(sigs[j]);
                count += 1;
            }
        }
        let avg = total / count as f64;
        // Max possible distance is sqrt(5) ≈ 2.236
        (avg / 5.0_f64.sqrt()).min(1.0)
    }

    /// Group agents into clusters by signature similarity.
    /// Uses single-linkage clustering with a default threshold of 0.5.
    pub fn clustering(&self) -> Vec<Vec<String>> {
        self.clustering_with_threshold(0.5)
    }

    /// Group agents into clusters by signature similarity with a custom threshold.
    pub fn clustering_with_threshold(&self, threshold: f64) -> Vec<Vec<String>> {
        let agents: Vec<&str> = self.signatures.keys().map(|s| s.as_str()).collect();
        if agents.is_empty() {
            return Vec::new();
        }

        // Union-Find for single-linkage clustering
        let n = agents.len();
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut Vec<usize>, i: usize) -> usize {
            if parent[i] != i {
                parent[i] = find(parent, parent[i]);
            }
            parent[i]
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let si = self.signatures.get(agents[i]).unwrap();
                let sj = self.signatures.get(agents[j]).unwrap();
                if si.distance_to(sj) <= threshold {
                    let ri = find(&mut parent, i);
                    let rj = find(&mut parent, j);
                    if ri != rj {
                        parent[ri] = rj;
                    }
                }
            }
        }

        let mut clusters: HashMap<usize, Vec<String>> = HashMap::new();
        for (i, agent) in agents.into_iter().enumerate() {
            let root = find(&mut parent, i);
            clusters.entry(root).or_default().push(agent.to_string());
        }

        clusters.into_values().collect()
    }

    pub fn registry_stats(&self) -> AdinkraStats {
        let total_symbols = self.symbols.len();
        let total_agents = self.signatures.len();
        let avg_complexity = if total_symbols > 0 {
            self.symbols.values().map(|s| s.complexity).sum::<f64>() / total_symbols as f64
        } else {
            0.0
        };
        let balanced_count = self.symbols.values().filter(|s| s.is_balanced()).count();
        let balanced_ratio = if total_symbols > 0 {
            balanced_count as f64 / total_symbols as f64
        } else {
            0.0
        };
        let clusters = self.clustering();
        AdinkraStats {
            total_symbols,
            total_agents,
            avg_complexity,
            balanced_ratio,
            cluster_count: clusters.len(),
        }
    }
}

// ---------------------------------------------------------------------------
// 7. Pre-built Adinkra symbols
// ---------------------------------------------------------------------------

pub fn sankofa() -> AdinkraSymbol {
    AdinkraSymbol {
        id: AdinkraId::new("sankofa"),
        name: "Sankofa".to_string(),
        meaning: SymbolMeaning {
            name: "Sankofa".to_string(),
            proverb: "Go back and fetch it".to_string(),
            teaching: "Learn from the past to build the future".to_string(),
            elements: 7,
        },
        symmetry_fold: 3,
        has_reflection: false,
        complexity: 0.65,
        geometry: SymbolGeometry::Spiral,
    }
}

pub fn gye_nyame() -> AdinkraSymbol {
    AdinkraSymbol {
        id: AdinkraId::new("gye-nyame"),
        name: "Gye Nyame".to_string(),
        meaning: SymbolMeaning {
            name: "Gye Nyame".to_string(),
            proverb: "Except God".to_string(),
            teaching: "Acknowledges the supremacy of the divine in all things".to_string(),
            elements: 12,
        },
        symmetry_fold: 8,
        has_reflection: true,
        complexity: 0.80,
        geometry: SymbolGeometry::Radial { arms: 8 },
    }
}

pub fn adinkrahene() -> AdinkraSymbol {
    AdinkraSymbol {
        id: AdinkraId::new("adinkrahene"),
        name: "Adinkrahene".to_string(),
        meaning: SymbolMeaning {
            name: "Adinkrahene".to_string(),
            proverb: "Chief of the Adinkra symbols".to_string(),
            teaching: "Greatness and leadership through character".to_string(),
            elements: 9,
        },
        symmetry_fold: 3,
        has_reflection: true,
        complexity: 0.55,
        geometry: SymbolGeometry::Concentric { rings: 3 },
    }
}

pub fn fawohodie() -> AdinkraSymbol {
    AdinkraSymbol {
        id: AdinkraId::new("fawohodie"),
        name: "Fawohodie".to_string(),
        meaning: SymbolMeaning {
            name: "Fawohodie".to_string(),
            proverb: "Independence comes with responsibility".to_string(),
            teaching: "Freedom is earned through accountability".to_string(),
            elements: 8,
        },
        symmetry_fold: 4,
        has_reflection: true,
        complexity: 0.70,
        geometry: SymbolGeometry::Interlocking { count: 4 },
    }
}

pub fn mate_masie() -> AdinkraSymbol {
    AdinkraSymbol {
        id: AdinkraId::new("mate-masie"),
        name: "Mate Masie".to_string(),
        meaning: SymbolMeaning {
            name: "Mate Masie".to_string(),
            proverb: "I have heard and kept it".to_string(),
            teaching: "True understanding requires deep listening and retention".to_string(),
            elements: 5,
        },
        symmetry_fold: 1,
        has_reflection: false,
        complexity: 0.45,
        geometry: SymbolGeometry::Asymmetric,
    }
}

/// Convenience: all pre-built symbols as a Vec.
pub fn builtin_symbols() -> Vec<AdinkraSymbol> {
    vec![sankofa(), gye_nyame(), adinkrahene(), fawohodie(), mate_masie()]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    extern crate serde_json;

    // --- AdinkraId ---
    #[test]
    fn test_adinkra_id_new() {
        let id = AdinkraId::new("test");
        assert_eq!(id.as_str(), "test");
    }

    #[test]
    fn test_adinkra_id_eq() {
        assert_eq!(AdinkraId::new("a"), AdinkraId::new("a"));
        assert_ne!(AdinkraId::new("a"), AdinkraId::new("b"));
    }

    #[test]
    fn test_adinkra_id_clone() {
        let id = AdinkraId::new("orig");
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_adinkra_id_hash_eq_consistent() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(AdinkraId::new("x"));
        assert!(set.contains(&AdinkraId::new("x")));
        assert!(!set.contains(&AdinkraId::new("y")));
    }

    #[test]
    fn test_adinkra_id_serde_roundtrip() {
        let id = AdinkraId::new("serde-test");
        let json = serde_json::to_string(&id).unwrap();
        let back: AdinkraId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
    }

    // --- SymbolMeaning ---
    #[test]
    fn test_symbol_meaning_serde() {
        let m = SymbolMeaning {
            name: "Test".into(),
            proverb: "p".into(),
            teaching: "t".into(),
            elements: 3,
        };
        let json = serde_json::to_string(&m).unwrap();
        let back: SymbolMeaning = serde_json::from_str(&json).unwrap();
        assert_eq!(back.elements, 3);
    }

    // --- AdinkraSymbol ---
    #[test]
    fn test_is_balanced_with_symmetry() {
        let sym = sankofa();
        assert!(sym.is_balanced()); // symmetry_fold=3
    }

    #[test]
    fn test_is_balanced_with_reflection() {
        let sym = mate_masie();
        assert!(!sym.is_balanced()); // fold=1, no reflection
    }

    #[test]
    fn test_is_balanced_fold_2() {
        let sym = AdinkraSymbol {
            id: AdinkraId::new("t"),
            name: "T".into(),
            meaning: SymbolMeaning {
                name: "T".into(),
                proverb: "p".into(),
                teaching: "t".into(),
                elements: 2,
            },
            symmetry_fold: 2,
            has_reflection: false,
            complexity: 0.5,
            geometry: SymbolGeometry::Radial { arms: 2 },
        };
        assert!(sym.is_balanced());
    }

    #[test]
    fn test_encode_complexity_spiral() {
        let sym = sankofa();
        let c = sym.encode_complexity();
        assert!((0.0..=1.0).contains(&c));
    }

    #[test]
    fn test_encode_complexity_radial() {
        let sym = gye_nyame();
        let c = sym.encode_complexity();
        assert!((0.0..=1.0).contains(&c));
    }

    #[test]
    fn test_encode_complexity_asymmetric() {
        let sym = mate_masie();
        let c = sym.encode_complexity();
        assert!((0.0..=1.0).contains(&c));
    }

    #[test]
    fn test_adinkra_symbol_serde() {
        let sym = sankofa();
        let json = serde_json::to_string(&sym).unwrap();
        let back: AdinkraSymbol = serde_json::from_str(&json).unwrap();
        assert_eq!(sym.id, back.id);
        assert_eq!(sym.name, back.name);
    }

    // --- AgentSignature ---
    #[test]
    fn test_signature_new_clamps() {
        let sig = AgentSignature::new("a", 1.5, -0.1, 0.5, 0.5, 0.5);
        assert!((sig.patience - 1.0).abs() < f64::EPSILON);
        assert!((sig.precision).abs() < f64::EPSILON);
    }

    #[test]
    fn test_distance_identical() {
        let a = AgentSignature::new("a", 0.5, 0.5, 0.5, 0.5, 0.5);
        assert!((a.distance_to(&a)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_distance_different() {
        let a = AgentSignature::new("a", 0.0, 0.0, 0.0, 0.0, 0.0);
        let b = AgentSignature::new("b", 1.0, 1.0, 1.0, 1.0, 1.0);
        let d = a.distance_to(&b);
        assert!(d > 0.0);
        // sqrt(5) ≈ 2.236
        assert!((d - 5.0_f64.sqrt()).abs() < 1e-9);
    }

    #[test]
    fn test_is_similar_within_threshold() {
        let a = AgentSignature::new("a", 0.5, 0.5, 0.5, 0.5, 0.5);
        let b = AgentSignature::new("b", 0.6, 0.5, 0.5, 0.5, 0.5);
        assert!(a.is_similar(&b, 0.2));
    }

    #[test]
    fn test_is_similar_outside_threshold() {
        let a = AgentSignature::new("a", 0.0, 0.0, 0.0, 0.0, 0.0);
        let b = AgentSignature::new("b", 1.0, 1.0, 1.0, 1.0, 1.0);
        assert!(!a.is_similar(&b, 1.0));
    }

    #[test]
    fn test_to_adinkra_patient_precise() {
        let sig = AgentSignature::new("wise-owl", 0.8, 0.9, 0.3, 0.2, 0.4);
        let adinkra = sig.to_adinkra();
        assert!(matches!(adinkra.geometry, SymbolGeometry::Radial { .. }));
        assert!(adinkra.is_balanced());
    }

    #[test]
    fn test_to_adinkra_playful_social() {
        let sig = AgentSignature::new("trickster", 0.2, 0.3, 0.8, 0.2, 0.9);
        let adinkra = sig.to_adinkra();
        assert!(matches!(adinkra.geometry, SymbolGeometry::Spiral));
    }

    #[test]
    fn test_to_adinkra_conservator() {
        let sig = AgentSignature::new("keeper", 0.3, 0.5, 0.2, 0.9, 0.3);
        let adinkra = sig.to_adinkra();
        assert!(matches!(adinkra.geometry, SymbolGeometry::Concentric { .. }));
    }

    #[test]
    fn test_to_adinkra_default_asymmetric() {
        let sig = AgentSignature::new("lone", 0.3, 0.3, 0.3, 0.3, 0.3);
        let adinkra = sig.to_adinkra();
        assert!(matches!(adinkra.geometry, SymbolGeometry::Asymmetric));
    }

    #[test]
    fn test_to_adinkra_interlocking() {
        let sig = AgentSignature::new("tinkerer", 0.3, 0.4, 0.7, 0.3, 0.2);
        let adinkra = sig.to_adinkra();
        assert!(matches!(adinkra.geometry, SymbolGeometry::Interlocking { .. }));
    }

    #[test]
    fn test_to_adinkra_complexity_range() {
        let sig = AgentSignature::new("x", 0.5, 0.5, 0.5, 0.5, 0.5);
        let a = sig.to_adinkra();
        assert!((0.0..=1.0).contains(&a.complexity));
    }

    #[test]
    fn test_signature_serde() {
        let sig = AgentSignature::new("test", 0.5, 0.6, 0.7, 0.8, 0.9);
        let json = serde_json::to_string(&sig).unwrap();
        let back: AgentSignature = serde_json::from_str(&json).unwrap();
        assert!((back.patience - 0.5).abs() < f64::EPSILON);
    }

    // --- AdinkraRegistry ---
    #[test]
    fn test_registry_register_symbol() {
        let mut reg = AdinkraRegistry::new();
        reg.register_symbol(sankofa());
        assert_eq!(reg.symbols.len(), 1);
    }

    #[test]
    fn test_registry_register_agent() {
        let mut reg = AdinkraRegistry::new();
        let sig = AgentSignature::new("agent-1", 0.5, 0.5, 0.5, 0.5, 0.5);
        reg.register_agent("agent-1", sig);
        assert_eq!(reg.signatures.len(), 1);
        assert!(reg.agent_symbol("agent-1").is_some());
    }

    #[test]
    fn test_registry_agent_symbol_missing() {
        let reg = AdinkraRegistry::new();
        assert!(reg.agent_symbol("nobody").is_none());
    }

    #[test]
    fn test_registry_find_similar() {
        let mut reg = AdinkraRegistry::new();
        let a = AgentSignature::new("a", 0.5, 0.5, 0.5, 0.5, 0.5);
        let b = AgentSignature::new("b", 0.55, 0.5, 0.5, 0.5, 0.5);
        let c = AgentSignature::new("c", 0.1, 0.1, 0.1, 0.1, 0.1);
        reg.register_agent("a", a);
        reg.register_agent("b", b);
        reg.register_agent("c", c);
        let similar = reg.find_similar_agents("a", 0.2);
        assert!(similar.contains(&"b"));
        assert!(!similar.contains(&"c"));
    }

    #[test]
    fn test_registry_find_similar_empty() {
        let reg = AdinkraRegistry::new();
        assert!(reg.find_similar_agents("ghost", 1.0).is_empty());
    }

    #[test]
    fn test_registry_diversity_single() {
        let mut reg = AdinkraRegistry::new();
        reg.register_agent("only", AgentSignature::new("only", 0.5, 0.5, 0.5, 0.5, 0.5));
        assert_eq!(reg.symbol_diversity(), 0.0);
    }

    #[test]
    fn test_registry_diversity_diverse() {
        let mut reg = AdinkraRegistry::new();
        reg.register_agent("a", AgentSignature::new("a", 0.0, 0.0, 0.0, 0.0, 0.0));
        reg.register_agent("b", AgentSignature::new("b", 1.0, 1.0, 1.0, 1.0, 1.0));
        let d = reg.symbol_diversity();
        assert!(d > 0.5);
    }

    #[test]
    fn test_registry_clustering() {
        let mut reg = AdinkraRegistry::new();
        reg.register_agent("a1", AgentSignature::new("a1", 0.1, 0.1, 0.1, 0.1, 0.1));
        reg.register_agent("a2", AgentSignature::new("a2", 0.15, 0.1, 0.1, 0.1, 0.1));
        reg.register_agent("b1", AgentSignature::new("b1", 0.9, 0.9, 0.9, 0.9, 0.9));
        let clusters = reg.clustering_with_threshold(0.5);
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn test_registry_clustering_empty() {
        let reg = AdinkraRegistry::new();
        assert!(reg.clustering().is_empty());
    }

    #[test]
    fn test_registry_stats() {
        let mut reg = AdinkraRegistry::new();
        reg.register_symbol(sankofa());
        reg.register_symbol(gye_nyame());
        reg.register_agent("a", AgentSignature::new("a", 0.5, 0.5, 0.5, 0.5, 0.5));
        let stats = reg.registry_stats();
        assert_eq!(stats.total_symbols, 3);
        assert_eq!(stats.total_agents, 1);
        assert!(stats.avg_complexity > 0.0);
        assert!(stats.balanced_ratio > 0.0);
    }

    // --- Pre-built symbols ---
    #[test]
    fn test_builtin_symbols_count() {
        assert_eq!(builtin_symbols().len(), 5);
    }

    #[test]
    fn test_sankofa() {
        let s = sankofa();
        assert_eq!(s.name, "Sankofa");
        assert!(matches!(s.geometry, SymbolGeometry::Spiral));
        assert_eq!(s.symmetry_fold, 3);
    }

    #[test]
    fn test_gye_nyame() {
        let s = gye_nyame();
        assert_eq!(s.name, "Gye Nyame");
        assert!(matches!(s.geometry, SymbolGeometry::Radial { arms: 8 }));
        assert!(s.has_reflection);
    }

    #[test]
    fn test_adinkrahene() {
        let s = adinkrahene();
        assert!(matches!(s.geometry, SymbolGeometry::Concentric { rings: 3 }));
    }

    #[test]
    fn test_fawohodie() {
        let s = fawohodie();
        assert!(matches!(s.geometry, SymbolGeometry::Interlocking { count: 4 }));
    }

    #[test]
    fn test_mate_masie() {
        let s = mate_masie();
        assert!(matches!(s.geometry, SymbolGeometry::Asymmetric));
        assert!(!s.is_balanced());
    }

    #[test]
    fn test_geometry_serde_variants() {
        let variants: Vec<SymbolGeometry> = vec![
            SymbolGeometry::Spiral,
            SymbolGeometry::Radial { arms: 4 },
            SymbolGeometry::Interlocking { count: 3 },
            SymbolGeometry::Concentric { rings: 2 },
            SymbolGeometry::Asymmetric,
        ];
        for v in &variants {
            let json = serde_json::to_string(v).unwrap();
            let back: SymbolGeometry = serde_json::from_str(&json).unwrap();
            assert_eq!(serde_json::to_string(v).unwrap(), serde_json::to_string(&back).unwrap());
        }
    }
}
